// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//

// Copilot suggestion plus minor adjustment '<Rhs = Self>' to make Rhs default to Self.:
pub trait Power<Rhs = Self> {
    type Output;
    fn power(self, rhs: Rhs) -> Self::Output;
}

impl Power<u16> for u32 {
    type Output = u32;
    fn power(self, rhs: u16) -> Self::Output {
        self.pow(rhs as u32)
    }
}

impl Power<u32> for u32 {
    type Output = u32;
    fn power(self, rhs: u32) -> Self::Output {
        self.pow(rhs)
    }
}

impl Power<&u32> for u32 {
    type Output = u32;
    fn power(self, rhs: &u32) -> Self::Output {
        self.pow(*rhs)
    }
}

// Output lets std decouple the implementor from the return type, thus supporting this case.

// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

// My initial attempt with generics that doesn't work because of conflicting
// implementations. This was not too far off though.

// pub trait Power<Rhs> {
//     type Output;
//     fn power(self, rhs: Rhs) -> Self::Output;
// }

// impl Power<u32> for u32 {
//     type Output = u32;
//     fn power(self, rhs: u16) -> Self::Output {
//         self.pow(rhs as u32)
//     }
// }

// impl Power<u32> for u32 {
//     type Output = u32;
//     fn power(self, rhs: u32) -> Self::Output {
//         self.pow(rhs)
//     }
// }

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
