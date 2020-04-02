//! And blood-black nothingness began to spin
//! A systems of cells interlinked within
//! Cells interlinked withing cells interlinked
//! Within on stem. And dreadfully distinct
//! Against the dark, a tall white fountain played.
enum Foo<X> {
    Bar { x: i32, y: X},
    Baz(i32),
}

fn main() {
    println!("Hello, {}!", 92);

    let mut vec = Vec::<Foo<_>>::new();
    if true {
        let x = 92;
        vec.push(Foo::Bar { x, y: 1});
    }
    unsafe { vec.set_len(0); }

    let mut spam = 42;
    let eggs = &mut spam;
    let ham = &eggs;
    eggs;
}
