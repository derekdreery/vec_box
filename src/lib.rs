
#[macro_export]
macro_rules! vec_box {
    ($elem:expr; $n:expr) => (
        $crate::vec::from_elem(Box::new($elem), $n)
    );
    ($($x:expr),*) => (
        <[_]>::into_vec(Box::new([$(Box::new($x)),*]))
    );
    ($($x:expr,)*) => (vec![$(Box::new($x)),*])
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut v: Vec<Box<u8>> = Vec::new();
        v.push(Box::new(1));
        v.push(Box::new(2));
        v.push(Box::new(3));
        assert_eq!(v, vec_box![1u8, 2, 3]);
    }

    // helpers for trait objects
    struct A {
        x: u32
    }

    impl A {
        fn new(x: u32) -> A {
            A { x: x }
        }
    }

    struct B {
        x: u32,
        y: u32
    }

    impl B {
        fn new(x: u32) -> B {
            B { x: x, y: x }
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

    #[test]
    fn trait_objects() {
        let v: Vec<Box<C>> = vec_box![
            A::new(1),
            B::new(1),
            A::new(2)
        ];

        assert_eq!(
            v.iter().fold(0, |acc, ref x| acc + x.get()),
            5
        );
    }
}
