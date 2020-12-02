pub mod template {
    use crate::{file_to_vec, Solve};

    pub struct Template {}

    impl Template {
        fn new() -> Self {
            Template {}
        }

        pub fn default() -> Self {
            Template::new()
        }
    }

    impl Solve for Template {
        fn solve(&self) {
            // TODO
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test() {
            assert!(true);
        }
    }
}
