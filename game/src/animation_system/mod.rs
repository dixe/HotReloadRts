use crate::types::*;
use gl_lib::animations::*;


#[allow(dead_code)]
#[derive(Debug)]
pub struct ActiveAnimation {
    animation: Animation, // list of key frames, and total duration
    current_time: Sec
}
