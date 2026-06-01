use uuid::Uuid;
use crate::domain::Passenger;
use crate::error::Result;
use crate::infrastructure::ReservationSystem;

pub struct BookingService<'a> {
    store: &'a mut ReservationSystem,
}

impl<'a> BookingService<'a> {
    pub fn new(store: &'a mut ReservationSystem) -> Self {
        Self { store }
    }

    pub fn create_booking(&mut self, passengers: Vec<Passenger>) -> Result<Uuid> {
        let booking = self.store.create_booking(passengers)?;
        Ok(booking.id)
    }
}
