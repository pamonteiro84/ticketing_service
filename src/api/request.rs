use serde_json::Value;
use crate::domain::{Passenger, SeatRef, Ticket};

pub fn parse_passengers(body: &Value) -> Result<Vec<Passenger>, String> {
    let passengers_json = body["passengers"]
        .as_array()
        .ok_or("missing 'passengers' array")?;

    passengers_json
        .iter()
        .map(|p| {
            let name = p["name"]
                .as_str()
                .ok_or("missing passenger name")?
                .to_string();
            let tickets_json = p["tickets"].as_array().ok_or("missing passenger tickets")?;
            let tickets = tickets_json
                .iter()
                .map(|t| {
                    let service_id = t["service_id"]
                        .as_str()
                        .ok_or("missing service_id")?
                        .to_string();
                    let seat_raw = t["seat"].as_str().ok_or("missing seat")?;
                    let seat_ref = SeatRef::parse(seat_raw).map_err(|e| e.to_string())?;
                    let origin = t["origin"].as_str().ok_or("missing origin")?.to_string();
                    let destination = t["destination"]
                        .as_str()
                        .ok_or("missing destination")?
                        .to_string();
                    Ok(Ticket { service_id, seat_ref, origin, destination })
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(Passenger { name, tickets })
        })
        .collect()
}
