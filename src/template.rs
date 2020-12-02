use crate::file_to_vec;

pub struct Template {}

impl Template {
    fn new() -> Self {
        Template {}
    }

    pub fn default() -> Self {
        Template::new()
    }

    fn solve(&self) -> i32 {
        // TODO
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        template = Template::default();
        assert!(template.solve() == 1);
    }
}
