// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

// The Drop trait is a mechanism for you to define additional cleanup logic for
// your types, beyond what the compiler does for you automatically. Whatever
// you put in the drop method will be executed when the value goes out of scope.

pub struct DropBomb {
    defused: bool,
}

impl DropBomb {
    pub fn new() -> Self {
        DropBomb { defused: false }
    }

    pub fn defuse(&mut self) {
        self.defused = true;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.defused {
            panic!("Boom! The drop bomb has exploded.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let _bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
