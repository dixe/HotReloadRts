extern crate shared;
use gl_lib::{gl, helpers};

fn main() {

    let sdl_setup = setup_window();

    let event_pump = &mut sdl_setup.sdl.event_pump().unwrap();
    let gl = &sdl_setup.gl;


    let mut game = load(gl);

    let mut last_change = std::fs::metadata("E:/repos/HotReloadRts/target/debug/game.dll").unwrap().modified().unwrap();

    loop {

        game.state.update_and_render(gl, event_pump);

        let cur_last_change = std::fs::metadata("E:/repos/HotReloadRts/target/debug/game.dll").unwrap().modified().unwrap();
        if cur_last_change > last_change {
            // Reload since new dll is more fresh
            // Drop is required for the new game to be replaced.


            drop(game);
            game = load(gl);

            last_change = std::fs::metadata("E:/repos/HotReloadRts/target/debug/game.dll").unwrap().modified().unwrap();
        }

        sdl_setup.window.gl_swap_window();
    }
}


fn setup_window() -> helpers::BasicSetup  {
    let sdl_setup = helpers::setup_sdl().unwrap();
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
