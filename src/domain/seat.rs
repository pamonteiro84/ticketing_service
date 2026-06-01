use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComfortClass {
    FirstClass,
    SecondClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SeatRef {
    pub carriage_id: String,
    pub seat_id: String,
}

impl SeatRef {
    pub fn parse(raw: &str) -> Result<Self, AppError> {
        let raw = raw.trim();
        if raw.is_empty() {
            return Err(AppError::InvalidSeatRef("seat ref must not be empty".to_string()));
        }

        let split_pos = raw
            .find(|c: char| c.is_ascii_digit())
            .ok_or_else(|| AppError::InvalidSeatRef(format!("no seat number in '{}'", raw)))?;

        if split_pos == 0 {
            return Err(AppError::InvalidSeatRef(format!(
                "no carriage letter in '{}'",
                raw
            )));
        }

        let carriage_id = raw[..split_pos].to_string();
        let seat_id = raw[split_pos..].to_string();

        Ok(SeatRef {
            carriage_id,
            seat_id,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seat {
    pub id: String,
    pub comfort: ComfortClass,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_seat_ref() {
        let seat = SeatRef::parse("A11").unwrap();
        assert_eq!(seat.carriage_id, "A");
        assert_eq!(seat.seat_id, "11");
    }

    #[test]
    fn parse_empty_returns_error() {
        assert!(SeatRef::parse("").is_err());
    }

    #[test]
    fn parse_no_carriage_returns_error() {
        assert!(SeatRef::parse("11").is_err());
    }

    #[test]
    fn parse_no_number_returns_error() {
        assert!(SeatRef::parse("A").is_err());
    }
}