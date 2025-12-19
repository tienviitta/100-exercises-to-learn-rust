// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

// If a type implements Copy, there's no need to call .clone() to create a new
// instance of the type: Rust does it implicitly for you.
// In most cases, you don't need to manually implement Copy. You can just
// derive it, like this

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

/*
I've implemented the necessary traits:
    Copy and Clone - Derived to enable implicit copying of WrappingU32 instances
    PartialEq - Derived to enable equality comparisons in assert_eq!
    Debug - Derived for better error messages
    Add - Manually implemented with wrapping_add to handle overflow (42 + 31 + 31 + u32::MAX wraps to 103)
The implementation uses wrapping arithmetic to handle the overflow in the test case correctly.
*/
impl std::ops::Add for WrappingU32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        WrappingU32 {
            value: self.value.wrapping_add(rhs.value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
