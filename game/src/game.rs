extern crate shared;
use crate::state;
use gl_lib::{gl, helpers, na, camera, widget_gui::{self, UiState}};
use crate::render;
use crate::handle_inputs;
use crate::commands::*;
use crate::reload;
use crate::loading;
use crate::ui;

//type ControllerType = camera::free_camera::Controller;
type ControllerType = camera::rts_camera::Controller;

// Store logic, which is lib and function points, seperate from the state. So we can reload logic. And call logic with &mut state
pub struct Game {
    pub gl: gl::Gl,

    //SIMLULATION/LOGIC
    pub state: state::State,
    pub logic: reload::Logic,

    // ALL RENDER DATA, LIKE SHADERS MESHES SHADOW MAP ECT.
    pub render_data: render::RenderData,

    // CAMERA
    pub camera: camera::Camera,
    pub camera_controller: ControllerType,


    // UI
    pub ui: Option<ui::Ui>,

    pub game_assets: loading::GameAssets,

    pub play_state: PlayState,

    // TMP BUFFERS
    pub tmp_buffer: Vec::<usize>

}


impl shared::SharedState for Game {
    fn update_and_render(&mut self, gl: &gl::Gl, event_pump: &mut gl_lib::sdl2::EventPump) {

        handle_inputs::handle_inputs(self, event_pump);

        self.camera_controller.update_camera(&mut self.camera, self.state.dt);

        // run logic step
        (self.logic.step_fn)(&mut self.state);


        // update ui
        if let Some(ui) = &mut self.ui {
            let  root_box = widget_gui::BoxContraint{ min_w: 0,
                                                     max_w: self.camera.width as i32,
                                                     min_h: 0,
                                                     max_h: self.camera.height as i32
            };


            widget_gui::layout_widgets(&root_box, &mut ui.state);
        }

        // render
       render::render(gl, self);
    }
}




#[derive(Clone, Copy, Debug)]
pub enum PlayState {
    General, // select and right
    ApplyCommand(Command)
}


#[derive(Copy, Clone, Debug)]
pub struct ScreenPos {
    pub x: i32,
    pub y: i32
}

#[derive(Copy, Clone, Debug)]
pub struct SelectBox {
    pub start: ScreenPos,
    pub current: ScreenPos,
}

pub struct Selection {
    pub active_box : Option::<SelectBox>,
}

impl SelectBox {

    pub fn min_x(&self) -> i32 {
        self.start.x.min(self.current.x)
    }

    pub fn max_x(&self) -> i32 {
        self.start.x.max(self.current.x)
    }

    pub fn min_y(&self) -> i32 {
        self.start.y.min(self.current.y)
    }

    pub fn max_y(&self) -> i32 {
        self.start.y.max(self.current.y)
    }

}


/// For Initializing from main Exe
#[no_mangle]
pub extern "Rust" fn initialize_state(gl: &gl::Gl) -> Box<dyn shared::SharedState> {

    // TODO: Figure out why when this is just above ui::create_ui(), the texture for font does not work
    // But setting it here it works fine with both this and shadow map texture

    let mut widget_setup = helpers::setup_widgets(gl).unwrap();

    let mut state = state::init();
    let logic = reload::load();

    let mut camera = camera::Camera::new(1200.0, 700.0);

    camera.move_to(na::Vector3::new(6.7, 6.5, 6.7));
    camera.look_at(na::Vector3::new(0.0, 0.0, 0.0));

    let mut camera_controller: ControllerType = Default::default();
    camera_controller.sens =  0.7;
    camera_controller.speed = 10.0;

    let base_path: std::path::PathBuf = "E:/repos/HotReloadRts/assets".to_string().into();


    let mut render_data = render::RenderData::new(gl, &base_path);

    let game_assets = loading::load_all_assets(base_path).unwrap();
    loading::populate_render_data(gl, &mut render_data, &game_assets.models);
    state::populate(&mut state, &game_assets, &render_data);

    let (info, ui_state) = ui::create_ui();

    let ui = ui::Ui {
        info,
        state: ui_state,
        widget_setup
    };


    Box::new(Game {
        gl: gl.clone(),
        state,
        camera,
        camera_controller,
        render_data,
        logic,
        game_assets,
        ui: Some(ui),
        tmp_buffer: vec![],
        play_state: PlayState::General,

    })
}


pub fn reset(game: &mut Game) {
    game.state = state::init();
    state::populate(&mut game.state, &game.game_assets, &game.render_data);
}


// Reloads all shaders, glb models and also logic.dll
pub fn reload_assets(game: &mut Game) {

    // maybe move this to a function in render
    let base_path: std::path::PathBuf = "E:/repos/HotReloadRts/assets".to_string().into();

    game.render_data.shaders.reload(&game.gl, &base_path);

    game.game_assets = loading::load_all_assets(base_path).unwrap();


    state::populate(&mut game.state, &game.game_assets, &game.render_data);

    game.logic = reload::load();

}
