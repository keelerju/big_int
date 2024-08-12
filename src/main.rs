use std::ops::{Deref, DerefMut};

fn main() {

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    enum Binary {
        #[default]
        Zero,
        One,
    }

    #[derive(Debug)]
    struct BigInt {
        inner: Vec<Binary>,
    }

    impl BigInt {
        fn new() -> Self {
            BigInt { inner: Vec::<Binary>::new() }
        }

        // method to convert the bits of a u32 into the inner vector of Binary values
        fn convert_from_integer(&mut self, mut n: u32) {
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

    // test of conversion with integer
    let mut bi = BigInt::new();
    bi.convert_from_integer(4956849);
    println!("{:?}", bi);

}
