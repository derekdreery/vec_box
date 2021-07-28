#[macro_use]
extern crate vec_box;

struct A {
    x: u32,
}

impl A {
    fn new(x: u32) -> A {
        A { x }
    }
}

struct B {
    x: u32,
    y: u32,
}

impl B {
    fn new(x: u32) -> B {
        B { x, y: x }
    }
}

trait C {
    fn get(&self) -> u32;
}

impl C for A {
    fn get(&self) -> u32 {
        self.x
    }
}

impl C for B {
    fn get(&self) -> u32 {
        self.x + self.y
    }
}

fn main() {
    let v: Vec<Box<dyn C>> = vec_box![A::new(1), B::new(1), A::new(2)];

    assert_eq!(v.iter().fold(0, |acc, ref x| acc + x.get()), 5);
}
