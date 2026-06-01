use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Station {
    pub name: String,
}

impl Station {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}