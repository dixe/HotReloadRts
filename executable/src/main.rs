extern crate shared;
use gl_lib::{gl, helpers};

#[cfg(feature = "no_reload")]
extern crate game as code;


fn main() {
    let sdl_setup = setup_window();

    let event_pump = &mut sdl_setup.sdl.event_pump().unwrap();
    let gl = &sdl_setup.gl;


    #[cfg(not(feature = "no_reload"))]
    let mut game = load(gl);

    #[cfg(feature = "no_reload")]
    let mut game = code::game::initialize_state(gl);

    #[cfg(not(feature = "no_reload"))]
    let mut last_change = std::fs::metadata("E:/repos/HotReloadRts/target/debug/game.dll").unwrap().modified().unwrap();

    // this make mouse be slower, so we need higher sens in controller.
    sdl_setup.sdl.mouse().set_relative_mouse_mode(true);


    loop {

        #[cfg(not(feature = "no_reload"))]
        game.state.update_and_render(gl, event_pump);

        #[cfg(feature = "no_reload")]
        game.update_and_render(gl, event_pump);

        #[cfg(not(feature = "no_reload"))]
        {
            panic!();
            let cur_last_change = std::fs::metadata("E:/repos/HotReloadRts/target/debug/game.dll").unwrap().modified().unwrap();

            if cur_last_change > last_change {
                // Reload since new dll is more fresh
                // Drop is required for the new game to be replaced.

                drop(game);
                game = load(gl);

                last_change = std::fs::metadata("E:/repos/HotReloadRts/target/debug/game.dll").unwrap().modified().unwrap();
            }

        }

        sdl_setup.window.gl_swap_window();
    }
}


fn setup_window() -> helpers::BasicSetup  {

    let sdl_setup = match helpers::setup_sdl() {
        Ok(ss) => ss,
        Err(e) => {
            panic!("Error in setup {:?}",e);
        }
    };

    let gl = sdl_setup.gl.clone();
    // Set background color to white
    // enable depth testing
    unsafe {
        gl.ClearColor(72.0/255.0, 130.0/255.0, 224.0/255.0, 1.0);
        gl.Enable(gl::DEPTH_TEST);
        gl.Enable(gl::CULL_FACE);
        gl.CullFace(gl::BACK);

        //gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    sdl_setup
}




fn load(gl: &gl::Gl) -> Game {
    let lib = shared::copy_and_load_lib("game.dll");

    let init_fn: libloading::Symbol<extern "Rust" fn(&gl::Gl) -> Box<dyn shared::SharedState>> =
        unsafe {
            lib.get(b"initialize_state")
        }.expect("Load of initi function");


    Game {
        state: init_fn(gl),
        _lib : lib
    }
}


struct Game {
    state: Box<dyn shared::SharedState>,
    _lib: libloading::Library
}
