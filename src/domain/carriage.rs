use serde::{Deserialize, Serialize};
use super::seat::Seat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Carriage {
    pub id: String,
    pub seats: Vec<Seat>,
}

impl Carriage {
    pub fn seat(&self, seat_id: &str) -> Option<&Seat> {
        self.seats.iter().find(|s| s.id == seat_id)
    }
}