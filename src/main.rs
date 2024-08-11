use std::ops::{Deref, DerefMut};

fn main() {

    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    enum Binary {
        #[default]
        Zero,
        One,
    }

    struct BigInt {
        inner: Vec<Binary>,
    }

    impl BigInt {
        fn new() -> Self {
            BigInt { inner: vec![Binary::default(); 32] }
        }
    }

    impl Deref for BigInt {
        type Target = Vec<Binary>;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl DerefMut for BigInt {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }

}
