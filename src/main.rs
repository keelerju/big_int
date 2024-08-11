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
            BigInt { inner: Vec::<Binary>::new() }
        }

        // method to convert the bits of a u32 into the inner vector of Binary values
        fn convert(&mut self, mut n: u32) {
            let mut binary_vec = Vec::<Binary>::new();

            while n > 0 {
                binary_vec.push(match n % 2 {
                    0 => Binary::Zero,
                    1 => Binary::One,
                    _ => return
                });
                n /= 2;
            }

            self.inner = binary_vec.iter().rev().cloned().collect();
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
