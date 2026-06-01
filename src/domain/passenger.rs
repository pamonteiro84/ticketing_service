use serde::{Deserialize, Serialize};
use super::ticket::Ticket;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passenger {
    pub name: String,
    pub tickets: Vec<Ticket>,
}

impl Passenger {
    pub fn new(name: &str, tickets: Vec<Ticket>) -> Self {
        Self { name: name.to_string(), tickets }
    }
}
