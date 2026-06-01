# Ticketing Service

A train seat reservation service built in Rust.

The REST interface is implemented via the `HttpClient` trait and exercised directly in integration tests — no HTTP server is required for this assignment.

## Run

```bash
make test     # run all tests
make clippy   # lint
make build    # release build
make fmt      # format
```

## Architecture

```
domain/          — pure entities, no dependencies
infrastructure/  — in-memory store, seed data
app/             — business logic (BookingService)
api/             — HttpClient trait, handlers, router
```

## Core Algorithm

Seat conflicts are detected using the `legs_overlap` formula:

```
overlap = o1 < d2 && o2 < d1
```

Where `o1/d1` is the requested leg and `o2/d2` is an already-booked leg, expressed as stop indices along the route.

| Case | o1 | d1 | o2 | d2 | Overlap? |
|------|----|----|----|----|----------|
| Paris→Amsterdam conflicts with Brussels→Amsterdam | 0 | 3 | 1 | 3 | ✅ `0 < 3 && 1 < 3` |
| Paris→Brussels does not conflict with Brussels→Amsterdam | 0 | 1 | 1 | 3 | ❌ `0 < 3 && 1 < 1` — false |

Adjacent legs (one ends exactly where the other begins) are **not** a conflict.

## Assumptions

- A multi-leg journey is represented as two separate tickets on two separate services, not as a single through-ticket.
- Seat availability is checked per service — there is no concept of a seat being shared across services.
- `RouteStop.distance_km` is the cumulative distance from the route origin, not the distance to the next stop.

## Scenarios

| # | Passenger(s) | Journey | Seats | Expected |
|---|--------------|---------|-------|----------|
| 1 | Alice + Bob | Paris → Amsterdam (5160) | A11, A12 | `201 Created` |
| 2 | Alice + Bob (repeat) | Paris → Amsterdam (5160) | A11, A12 | `409 Conflict` |
| 3 | Carol + Dave | London→Paris (6001) + Paris→Amsterdam (6002) | H1, N5 / A1, T7 | `201 Created` |
| 4 | Carol + Dave (repeat) | London→Paris (6001) + Paris→Amsterdam (6002) | H1, N5 / A1, T7 | `409 Conflict` |

## Going to Production

### Thread safety

Wrap `ReservationSystem` in `Arc<Mutex<>>` inside `Router`. Only `router.rs` changes — all other layers are unaffected:

```rust
pub struct Router {
    store: Arc<Mutex<ReservationSystem>>,
}
```

`HttpClient::post` takes `&self` and acquires the lock internally. The `Router` can then be cloned freely across threads.

### Real database

Replace `ReservationSystem` with a struct that holds a connection pool (e.g. `sqlx::PgPool`). The `app/` and `domain/` layers stay unchanged — only `infrastructure/` gets a new implementation. Consider extracting a `Store` trait so the app layer depends on the abstraction, not the concrete type.

### HTTP server

Drop an Axum or Actix router on top of `api/router.rs`. Each HTTP handler calls `router.post(url, body)` or `router.get(url)` and serialises the `Response` back to the client.
