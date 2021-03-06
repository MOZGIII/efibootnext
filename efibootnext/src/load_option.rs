use std::fmt;

#[derive(Debug, Clone)]
pub struct LoadOption {
    pub number: u16,
    pub description: String,
}

impl LoadOption {
    pub fn new(number: u16, description: String) -> Self {
        Self {
            number: number,
            description: description,
        }
    }
}

impl fmt::Display for LoadOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
