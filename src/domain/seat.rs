use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum AppError {
    InvalidSeatRef(String),
}

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