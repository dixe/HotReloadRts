use gl_lib::gl;
use random_string::generate;


/// Trait that the main exe can get a box(dyn ) to, and call the update. While being hot reloadable
pub trait SharedState {
    fn update_and_render(&mut self, gl: &gl::Gl, event_pump: &mut gl_lib::sdl2::EventPump);
}


pub fn copy_and_load_lib(name: &str) -> libloading::Library {

    let cur_path = std::env::current_exe().unwrap();
    let base_path = cur_path.parent().unwrap();

    let from_path = base_path.join(name);


    // generate a new name and path where so we can still compile the dll from
    let charset = "1234567890abcdefg";
    let ending = generate(6, charset);
    let new_path = base_path.join(name.replace(".dll", &format!("_{}.dll", ending)));

    let mut iter = 0;
    while let Err(_) = std::fs::copy(&from_path, &new_path) {
        iter += 1;
        if iter > 8000 {
            println!("Waiting on copy from '{:?}' to {:?}", from_path, new_path);
            iter = 0;
        }
    }


    let mut lib = unsafe {libloading::Library::new(&new_path) };
    while let Err(e) =  lib {
        iter += 1;
        if iter > 10000 {
            println!("Error loading lib {:?}", e);
            iter = 0;
        }
        unsafe {
            lib = libloading::Library::new(&new_path);
        }
    }

    lib.unwrap()

}
