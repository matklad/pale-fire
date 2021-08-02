// And blood-black nothingness began to spin
// A system of cells interlinked within
// Cells interlinked within cells interlinked
// Within one stem. And dreadfully distinct
// Against the dark, a tall white fountain played.
use std::ops::Add;

pub struct Enemy {
    name: &'static str,
    pub x: f32,
    pub y: f32,
    id: usize,
}

#[repr(transparent)]
struct Wrapper<T> {
    value: T,
}

impl Add<f32> for Wrapper<f32> {
    type Output = Wrapper<f32>;
    fn add(self, rhs: f32) -> Self::Output {
        todo!("not yet implemented :(")
    }
}

fn frobnicate<'a>(wrapper: &'a mut Wrapper<f32>) -> &'a f32 {
    wrapper.value += 1.0;
    unsafe { &*(wrapper as *const _) }
}
