# rust-investment-wallet

A web application for tracking investment portfolios, built with Rust. Users can register and log in to manage their owned assets, record purchases, and visualize P&L history through a server-rendered interface.

The project was developed during the **Santander Bootcamp Rust AI Developer** at [DIO](https://www.dio.me/).

---

## Features

- **Authentication** — Login and registration through a single form; JWT issued as an `HttpOnly` cookie on success
- **Asset catalog** — Admin-managed list of available assets (name + current unit value)
- **Portfolio view** — Per-user dashboard showing owned assets, total quantity, current unit value, and aggregate P&L
- **Purchase history** — Each asset row expands to show individual purchase records with date, quantity, price paid, and per-purchase P&L
- **Admin API** — Protected endpoints to create and update assets, authenticated via a static `Authorization` header

---

## Tech stack

| Crate | Purpose |
|---|---|
| [axum](https://github.com/tokio-rs/axum) `0.8` | HTTP framework and routing |
| [tokio](https://tokio.rs) | Async runtime (`rt-multi-thread`) |
| [askama](https://github.com/djc/askama) `0.16` | Compile-time HTML templating |
| [axum-extra](https://docs.rs/axum-extra) | Signed cookie jar extractor |
| [jwt-simple](https://github.com/jedisct1/rust-jwt-simple) | HS256 JWT generation and verification |
| [password-auth](https://github.com/RustCrypto/password-hashes) | Password hashing and verification (PHC string format) |
| [serde](https://serde.rs) / serde_json | JSON serialization/deserialization |
| [time](https://docs.rs/time) | Typed timestamps with ISO 8601 serde support |
| [thiserror](https://github.com/dtolnay/thiserror) | Ergonomic error type derivation |
| [tracing](https://docs.rs/tracing) / tracing-subscriber | Structured logging and span instrumentation |
| [color-eyre](https://github.com/yaahc/color-eyre) | Rich error reporting at startup |

---

## Project structure

```
src/
├── main.rs           # Entry point — starts the Tokio runtime
├── app.rs            # AppState (in-memory store) and server bootstrap
├── models.rs         # Domain types: Asset, UserRecord, PurchaseRecord, OwnedAsset
├── repository.rs     # Data access layer; implements FromRequestParts for injection
├── error.rs          # AppError enum with automatic HTTP status mapping
├── auth/
│   ├── user.rs       # User extractor (JWT from cookie), UnauthenticatedUser, token logic
│   └── admin.rs      # Admin extractor (static Authorization header)
└── routes/
    ├── api.rs        # Admin-only REST API: GET/POST/PATCH /api/assets
    └── frontend.rs   # Server-rendered pages: login, portfolio dashboard, purchase form

templates/
├── login.html        # Login/registration page (Askama + Tailwind CDN)
└── assets.html       # Portfolio dashboard with expandable purchase history
```

State is held entirely in memory using `Arc<Mutex<Vec<T>>>`, shared across handlers via Axum's `State` extractor.

---

## Running locally

```bash
# Prerequisites: Rust stable (edition 2024)
git clone https://github.com/<your-username>/rust-investment-wallet.git
cd rust-investment-wallet
cargo run
```

The server starts on **`http://localhost:3000`**.

> State is in-memory only — all data is lost on restart.

---

## Usage

### Web interface

| Route | Description |
|---|---|
| `GET /` | Redirects to `/assets` if authenticated, otherwise to `/login` |
| `GET /login` | Login / registration page |
| `POST /login` | Submits credentials; registers the user if the username is new |
| `GET /assets` | Portfolio dashboard (requires auth cookie) |
| `POST /assets` | Records a purchase (requires auth cookie) |
| `GET /logout` | Clears the auth cookie and redirects to `/login` |

### Admin REST API

All `/api/*` routes require the `Authorization` header set to the admin secret key.

#### List assets

```bash
curl http://localhost:3000/api/assets
```

```json
[
  { "id": 1, "name": "PETR4", "unit_value": 38.50 },
  { "id": 2, "name": "BTC",   "unit_value": 320000.00 }
]
```

#### Create asset

```bash
curl -X POST http://localhost:3000/api/assets \
  -H "Authorization: im-the-admin" \
  -H "Content-Type: application/json" \
  -d '{ "name": "PETR4", "unit_value": 38.50 }'
```

```json
{ "id": 1, "name": "PETR4", "unit_value": 38.50 }
```

#### Update asset

Supports partial updates — omit any field to leave it unchanged.

```bash
curl -X PATCH http://localhost:3000/api/assets \
  -H "Authorization: im-the-admin" \
  -H "Content-Type: application/json" \
  -d '{ "id": 1, "unit_value": 41.20 }'
```

```json
{ "id": 1, "name": "PETR4", "unit_value": 41.20 }
```

### Error responses

All errors return a consistent JSON body:

```json
{ "error": "Asset does not exist" }
```

| Scenario | HTTP status |
|---|---|
| Missing `Authorization` header / cookie | `400 Bad Request` |
| Wrong credentials or invalid JWT | `401 Unauthorized` |
| Asset or user not found | `404 Not Found` |
| Username already registered | `400 Bad Request` |

---

## Architecture notes

- **`Repository` as an Axum extractor** — `Repository` implements `FromRequestParts<AppState>`, so it is injected directly into handler signatures without requiring an explicit `State(...)` call.
- **`User` as an Axum extractor** — The `User` struct implements `FromRequestParts`, reading the `token` cookie and verifying the JWT. Routes that require auth simply declare `user: User` in their signature; unauthenticated requests are rejected automatically.
- **`Option<User>` extractor** — Used on the index route to redirect without returning an error for unauthenticated requests.
- **Compile-time templates** — Askama templates are type-checked at compile time against the structs they receive, eliminating a class of runtime template errors.

---

## Development

There are no automated tests in the current version. To check formatting and lints:

```bash
cargo fmt --check   # formatting
cargo clippy        # lints
cargo build         # full compilation check
```

---

## Context

This project was built as part of the **Santander Bootcamp Rust AI Developer**, a training program offered by [DIO](https://www.dio.me/). It served as a hands-on exercise in building a real, layered Rust web application — covering async programming with Tokio, HTTP routing with Axum, authentication with JWTs and password hashing, and server-side rendering with Askama.
