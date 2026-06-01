use serde::{Deserialize, Serialize};
use super::ticket::Ticket;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passenger {
    pub name: String,
    pub tickets: Vec<Ticket>,
}
