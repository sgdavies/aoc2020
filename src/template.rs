use crate::file_to_vec;

pub struct Template {
    _data: Vec<String>,
}

impl Template {
    fn new(filename: &str) -> Self {
        Template {
            _data: file_to_vec(filename),
        }
    }

    pub fn _default() -> Self {
        Template::new("data/template.txt")
    }

    pub fn solve(&self) -> i32 {
        // TODO
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let template = Template::new("data/1a.txt");
        assert!(template.solve() == 0);
    }
}
