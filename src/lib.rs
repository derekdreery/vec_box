#[macro_export]
macro_rules! vec_box {
    ($elem:expr; $n:expr) => (vec![Box::new($elem); $n]);
    ($($x:expr),*) => (vec![$(Box::new($x)),*]);
    ($($x:expr,)*) => (vec_box![$($x),*]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn macro_list() {
        assert_eq!(
            vec![Box::new(1), Box::new(2), Box::new(3)],
            vec_box![1u8, 2, 3]
        );
    }

    #[test]
    fn macro_list_trailing_comma() {
        assert_eq!(
            vec![Box::new(1), Box::new(2), Box::new(3)],
            vec_box![1u8, 2, 3,]
        );
    }

    #[test]
    fn macro_expr_count() {
        assert_eq!(vec![Box::new(1u8); 3], vec_box![1u8; 3]);
    }

    // helpers for trait objects
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

    #[test]
    fn trait_objects() {
        let v: Vec<Box<dyn C>> = vec_box![A::new(1), B::new(1), A::new(2)];

        assert_eq!(v.iter().fold(0, |acc, ref x| acc + x.get()), 5);
    }
}
