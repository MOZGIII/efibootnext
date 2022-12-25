//! The load option.

/// A possible load option value.
#[derive(Debug, Clone)]
pub struct LoadOption {
    /// The number of the load option.
    pub number: u16,
    /// The description for display.
    pub description: String,
}

impl std::fmt::Display for LoadOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}
