extern crate shared;
use nalgebra::vector;
use libloading;
use crate::state::*;
use gl_lib::{gl, na, objects::{plane, mesh, shadow_map, texture_quad, gltf_mesh}, shader::{self, Shader}, camera};
use gl_lib::controller;
use gl_lib::sdl2::{self, keyboard::Keycode};
use crate::render;

// Store logic, which is lib and function points, seperate from the state. So we can reload logic. And call logic with &mut state
pub struct Game {
    pub gl: gl::Gl,
    //SIMLULATION/LOGIC
    pub state: State,
    pub logic: Logic,

    pub select_box: Option<SelectBox>,

    // ALL RENDER DATA, LIKE SHADERS MESHES SHADOW MAP ECT.
    pub render_data: render::RenderData,

    // CAMERA
    pub camera: camera::Camera,
    pub camera_controller: camera::free_camera::Controller,
}


impl shared::SharedState for Game {
    fn update_and_render(&mut self, gl: &gl::Gl, event_pump: &mut gl_lib::sdl2::EventPump) {

        handle_inputs(self, event_pump);


        self.camera_controller.update_camera(&mut self.camera, self.state.dt);

        // run logic step
        (self.logic.step_fn)(&mut self.state);


        // render
       render::render(gl, self);
    }
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



pub fn load() -> Logic {
    let lib = shared::copy_and_load_lib("logic.dll");

    let step_fn: libloading::Symbol<extern "Rust" fn(&mut State)> =
        unsafe {
           lib.get(b"step")
        }.expect("Load of step fn");

    Logic {
        step_fn: *step_fn,
        _lib : lib,
    }
}


pub fn handle_inputs(game: &mut Game, event_pump: &mut gl_lib::sdl2::EventPump) {
    let kb_map = setup_keyboard_mapping();

    for event in event_pump.poll_iter() {
        game.camera_controller.update_events(event.clone());
        controller::on_input(event.clone(), &kb_map, game);

        use sdl2::event::Event::*;
        match event.clone() {
            MouseButtonDown{mouse_btn, x, y, ..} => {
                if game.select_box.is_none() && mouse_btn == sdl2::mouse::MouseButton::Left{
                    game.select_box = Some(SelectBox {
                        start: ScreenPos { x,y },
                        current: ScreenPos { x,y },
                    });
                }
            },
            MouseMotion{mousestate, x, y, .. } => {
                if let Some(sb) = &mut game.select_box {
                    sb.current.x = x;
                    sb.current.y = y;
                }
            },
            MouseButtonUp{mouse_btn, x, y, ..} => {
                game.select_box = None;
            },
            _ => {}
        }

    }
}



/// For Initializing from main Exe
#[no_mangle]
pub extern "Rust" fn initialize_state(gl: &gl::Gl) -> Box<dyn shared::SharedState> {

    let state = init();
    let logic = load();



    let mut camera = camera::Camera::new(1200.0, 700.0);
    camera.move_to(na::Vector3::new(5.0, 2.0, 3.0));
    camera.look_at(na::Vector3::new(0.0, 0.0, 0.0));


    Box::new(Game {
        gl: gl.clone(),
        state,
        camera,
        camera_controller: Default::default(),
        render_data: render::RenderData::new(gl),
        select_box: None,
        logic,

    })
}


pub struct Logic {
    step_fn: fn(&mut State),
    _lib: libloading::Library
}





fn setup_keyboard_mapping() -> controller::ControllerMapping<Game> {
    let mut kb_map = controller::ControllerMapping::new();

    use Keycode::*;
    kb_map.exit(Keycode::Escape);
    kb_map.add_on_press(R, reload_assets);
    kb_map.add_on_press(Q, reset);

    kb_map
}

fn reset(game: &mut Game) {
    game.state = init();
}


// Reloads all shaders, glb models and also logic.dll
fn reload_assets(game: &mut Game) {

    // maybe move this to a function in render
    let base_path: std::path::PathBuf = "E:/repos/HerdGame/assets".to_string().into();
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
            println!("Reloading selec_box shader");
        },
        Err(err) => {
            println!("{:?}", err);
        },
    };


    let hashmap = std::collections::HashMap::new();
    match gltf_mesh::meshes_from_gltf(&"E:/repos/HerdGame/assets/boid.glb", &hashmap) {
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
