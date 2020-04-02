//! And blood-black nothingness began to spin
//! A systems of cells interlinked within
//! Cells interlinked withing cells interlinked
//! Within on stem. And dreadfully distinct
//! Against the dark, a tall white fountain played.
use std::ops::Add;

#[repr(transparent)]
struct Wrapper<T> {
    value: T
}

impl Add<f32> for Wrapper<f32> {
    type Output = Wrapper<f32>;
    fn add(self, rhs: f32) -> Self {
        todo!("not yet implemented :(")
    }
}

fn frobnicate<'a>(wrapper: &'a mut Wrapper<f32>) -> &'a f32 {
    wrapper.value += 1.0;
    unsafe {
        &*(wrapper as *const _)
    }
}
