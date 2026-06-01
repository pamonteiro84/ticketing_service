use serde::{Deserialize, Serialize};
use super::seat::SeatRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub service_id: String,
    pub seat_ref: SeatRef,
    pub origin: String,
    pub destination: String,
}
