use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::{Booking, Passenger, Service, Ticket};
use crate::error::{AppError, Result};

pub struct ReservationSystem {
    services: HashMap<String, Service>,
    bookings: HashMap<Uuid, Booking>,
}

impl ReservationSystem {
    pub fn new(services: Vec<Service>) -> Self {
        Self {
            services: services.into_iter().map(|s| (s.id.clone(), s)).collect(),
            bookings: HashMap::new(),
        }
    }

    pub fn get_service(&self, service_id: &str) -> Result<&Service> {
        self.services
            .get(service_id)
            .ok_or_else(|| AppError::ServiceNotFound(service_id.to_string()))
    }

    pub fn get_booking(&self, id: Uuid) -> Result<&Booking> {
        self.bookings.get(&id).ok_or(AppError::BookingNotFound(id))
    }

    /// Validates all tickets before persisting. Either the entire booking is created or
    /// nothing is written — partial state is never stored.
    pub fn create_booking(&mut self, passengers: Vec<Passenger>) -> Result<&Booking> {
        for passenger in &passengers {
            for ticket in &passenger.tickets {
                self.validate_ticket(ticket)?;
            }
        }
        let booking = Booking::new(passengers);
        let id = booking.id;
        Ok(self.bookings.entry(id).or_insert(booking))
    }

    /// Checks seat existence and detects leg overlap against every existing booking
    /// for the same seat on the same service. A conflict exists when the requested
    /// leg overlaps any already-reserved leg: `o1 < d2 && o2 < d1`.
    fn validate_ticket(&self, ticket: &Ticket) -> Result<()> {
        let service = self.get_service(&ticket.service_id)?;
        let route = &service.route;

        let origin_idx = route
            .stop_index(&ticket.origin)
            .ok_or_else(|| AppError::StationNotFound {
                service_id: ticket.service_id.clone(),
                station: ticket.origin.clone(),
            })?;

        let dest_idx = route
            .stop_index(&ticket.destination)
            .ok_or_else(|| AppError::StationNotFound {
                service_id: ticket.service_id.clone(),
                station: ticket.destination.clone(),
            })?;

        if origin_idx >= dest_idx {
            return Err(AppError::InvalidLeg {
                origin: ticket.origin.clone(),
                destination: ticket.destination.clone(),
            });
        }

        let seat_label = format!("{}{}", ticket.seat_ref.carriage_id, ticket.seat_ref.seat_id);

        let carriage = service
            .carriages
            .iter()
            .find(|c| c.id == ticket.seat_ref.carriage_id)
            .ok_or_else(|| AppError::SeatNotFound {
                service_id: ticket.service_id.clone(),
                seat_ref: seat_label.clone(),
            })?;

        carriage.seat(&ticket.seat_ref.seat_id).ok_or_else(|| AppError::SeatNotFound {
            service_id: ticket.service_id.clone(),
            seat_ref: seat_label.clone(),
        })?;

        for booking in self.bookings.values() {
            for passenger in &booking.passengers {
                for existing in &passenger.tickets {
                    if existing.service_id != ticket.service_id {
                        continue;
                    }
                    if existing.seat_ref != ticket.seat_ref {
                        continue;
                    }
                    let Some(eo) = route.stop_index(&existing.origin) else { continue; };
                    let Some(ed) = route.stop_index(&existing.destination) else { continue; };
                    if crate::domain::Route::legs_overlap(origin_idx, dest_idx, eo, ed) {
                        return Err(AppError::SeatAlreadyTaken {
                            service_id: ticket.service_id.clone(),
                            seat_ref: seat_label,
                        });
                    }
                }
            }
        }

        Ok(())
    }
}
