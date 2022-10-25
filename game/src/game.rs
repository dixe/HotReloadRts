extern crate shared;
use libloading;
use crate::state::*;
use gl_lib::{gl, na, objects::{gltf_mesh, square}, camera};
use crate::render;
use crate::handle_inputs;
use crate::commands::*;
use crate::reload::*;

//type ControllerType = camera::free_camera::Controller;
type ControllerType = camera::rts_camera::Controller;

// Store logic, which is lib and function points, seperate from the state. So we can reload logic. And call logic with &mut state
pub struct Game {
    pub gl: gl::Gl,

    //SIMLULATION/LOGIC
    pub state: State,
    pub logic: Logic,

    // ALL RENDER DATA, LIKE SHADERS MESHES SHADOW MAP ECT.
    pub render_data: render::RenderData,

    // CAMERA
    pub camera: camera::Camera,
    pub camera_controller: ControllerType,


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

    let state = init();
    let logic = load();


    let mut camera = camera::Camera::new(1200.0, 700.0);

    camera.move_to(na::Vector3::new(6.7, 6.5, 6.7));
    camera.look_at(na::Vector3::new(0.0, 0.0, 0.0));

    let mut camera_controller: ControllerType = Default::default();
    camera_controller.sens =  0.7;
    camera_controller.speed = 10.0;

    Box::new(Game {
        gl: gl.clone(),
        state,
        camera,
        camera_controller,
        render_data: render::RenderData::new(gl),
        logic,

        tmp_buffer: vec![],
        play_state: PlayState::General,

    })
}


pub fn reset(game: &mut Game) {
    game.state = init();
}


// Reloads all shaders, glb models and also logic.dll
pub fn reload_assets(game: &mut Game) {

    // maybe move this to a function in render
    let base_path: std::path::PathBuf = "E:/repos/HotReloadRts/assets".to_string().into();

    match render::create_shader(&game.gl, &base_path, "mesh") {
        Ok(shader) => {
            game.render_data.mesh_shader = shader;
            println!("Reloaded mesh shader");
        },
        Err(err) => {
            println!("{:?}", err);
        },
    };

    match render::create_shader(&game.gl, &base_path, "select_box") {
        Ok(shader) => {
            game.render_data.select_box_shader = shader;
            println!("Reloaded selec_box shader");
        },
        Err(err) => {
            println!("{:?}", err);
        },
    };

    match render::create_shader(&game.gl, &base_path, "hp") {
        Ok(shader) => {
            game.render_data.hp_shader = shader;
            println!("Reloaded hp shader");
        },
        Err(err) => {
            println!("{:?}", err);
        },
    };



    // Kind of a hack, since not recreating this make binding the vbo and sub data fail with gl 1282
    game.render_data.square = square::Square::new(&game.gl);

    let hashmap = std::collections::HashMap::new();
    match gltf_mesh::meshes_from_gltf(&"E:/repos/HotReloadRts/assets/boid.glb", &hashmap) {
        Ok(boid_gltf) => {
            match boid_gltf.get_mesh(&game.gl, "Boid") {
                Some(boid) => {
                    game.render_data.boid = boid;
                    println!("Reloaded boid");
                },
                None => {
                    println!("Could not get boid from gltf_mesh");
                }
            }
        },
        Err(err) => {
            println!("{:?}",err);
        }
    }

    game.logic = load();

}
