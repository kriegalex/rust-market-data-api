# rust-async-project

A market data REST API inspired by exchange APIs (Kraken-style), built with Axum, Tokio, and Tower.

## Features

- Ticker and order book endpoints for crypto trading pairs
- Tower middleware for Prometheus metrics (request count + latency)
- Structured JSON tracing via `tracing-subscriber`
- `rust_decimal` for lossless price/quantity representation

## Setup

```sh
cargo build
cargo run
```

The server listens on `0.0.0.0:3000` by default.

Set log level via the `RUST_LOG` environment variable:

```sh
RUST_LOG=info cargo run
```

## Endpoints

### Health

```sh
curl http://localhost:3000/health
```

```json
{"status":"ok","uptime_secs":12,"symbols":["BTCUSD","ETHUSD","SOLUSD"]}
```

### Ticker

```sh
curl http://localhost:3000/ticker/BTCUSD
```

```json
{"symbol":"BTCUSD","bid":"67450.00","ask":"67451.50","last":"67450.75","volume_24h":"1234.567"}
```

Symbols are case-insensitive: `/ticker/btcusd` and `/ticker/BTCUSD` return the same result.

### Order Book

```sh
curl http://localhost:3000/orderbook/BTCUSD
```

```json
{
  "symbol": "BTCUSD",
  "bids": [
    {"price":"67450.00","quantity":"0.5"},
    {"price":"67449.00","quantity":"1.2"},
    {"price":"67445.00","quantity":"2.0"}
  ],
  "asks": [
    {"price":"67451.50","quantity":"0.3"},
    {"price":"67452.00","quantity":"0.8"},
    {"price":"67455.00","quantity":"1.5"}
  ]
}
```

Bids are sorted descending by price; asks ascending.

### Prometheus Metrics

```sh
curl http://localhost:3000/metrics
```

Returns a Prometheus text exposition of `http_requests_total` and `http_request_duration_seconds`.

## Available Symbols

`BTCUSD`, `ETHUSD`, `SOLUSD`

Unknown symbols return `404`:

```sh
curl -i http://localhost:3000/ticker/FAKEUSD
# HTTP/1.1 404 Not Found
# {"error":"symbol not found: FAKEUSD"}
```

## Tests

```sh
cargo test
```
