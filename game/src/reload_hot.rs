use crate::state::*;
use libloading;

pub struct Logic {
    pub step_fn: fn(&mut State),
    _lib: libloading::Library
}

pub fn load() -> Logic {
    let lib = shared::copy_and_load_lib("logic.dll");

    let step_fn: libloading::Symbol<extern "Rust" fn(&mut State)> =
        unsafe {
            lib.get(b"step")
        }.expect("Load of step fn");

    println!("Reloaded lib");
    Logic {
        step_fn: *step_fn,
        _lib : lib,
    }
}
