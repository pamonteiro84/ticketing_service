use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::passenger::Passenger;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    pub id: Uuid,
    pub passengers: Vec<Passenger>,
}

impl Booking {
    pub fn new(passengers: Vec<Passenger>) -> Self {
        Self {
            id: Uuid::new_v4(),
            passengers,
        }
    }
}
