pub type Sec = f32;

#[derive(Debug)]
pub struct SliceInfo<T> {
    pub pointer: *const T,
    pub len: usize
}
