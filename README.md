# 🦀 OnlineCrab

OnlineCrab is a minimal, high-performance real-time WebSocket presence
server built with **Rust + Actix-Web**.

It tracks how many clients are currently connected and broadcasts the
latest online count to admin clients in real time.

This project demonstrates a clean and scalable approach to handling
presence state using `tokio::watch` instead of event-based broadcast
channels.

------------------------------------------------------------------------

## 🚀 Installation
-   Make sure you have .env file please copy it from .env.example

------------------------------------------------------------------------

## 🚀 Features

-   Real-time online client counter
-   Admin clients receive live updates
-   Clients are counted per WebSocket connection
-   Admin connections are not counted
-   Uses `tokio::watch` to avoid dropped state updates
-   Async WebSocket handling via `actix-ws`
-   Clean, minimal, production-ready structure

------------------------------------------------------------------------

## 🏗 Architecture Overview

OnlineCrab uses:

-   `actix-web` for HTTP server
-   `actix-ws` for WebSocket handling
-   `tokio::watch` for state broadcasting
-   `AtomicUsize` for concurrent-safe counter

### Why `watch` instead of `broadcast`?

Presence count is a **state problem**, not an event stream problem.

-   `broadcast` sends every event and can drop messages under load.
-   `watch` always keeps the latest value.
-   Admin clients always receive the current online count without lag or
    loss.

This makes the system stable even under heavy load testing.

------------------------------------------------------------------------

## 📦 Dependencies

``` toml
actix-web = "4"
actix-ws = "0.2"
tokio = { version = "1", features = ["sync", "macros", "rt-multi-thread"] }
serde = { version = "1", features = ["derive"] }
futures-util = "0.3"
```

------------------------------------------------------------------------

## ▶️ Running the Server

``` bash
cargo run
```

Server will start at:

    http://127.0.0.1:3000

------------------------------------------------------------------------

## 🌐 WebSocket Endpoints

    /ws?role=client
    /ws?role=admin

### Client

Each client connection: - Increments the counter - Decrements on
disconnect

### Admin

Admin connection: - Does NOT affect the counter - Receives live online
count updates

------------------------------------------------------------------------

## 🧪 Testing

### Manual Test

Open multiple browser tabs:

    client.html

Open:

    admin.html

You should see the online counter update in real time.

------------------------------------------------------------------------

### Load Testing Example (k6)

``` bash
k6 run --vus 1000 --duration 30s script.js
```

OnlineCrab handles high churn safely because:

-   State is centralized via `watch`
-   No event backlog
-   No message loss

------------------------------------------------------------------------

## 🧠 Design Philosophy

OnlineCrab intentionally keeps the presence model simple:

-   Online = WebSocket connected
-   Offline = Connection closed or dropped
-   Admin receives the latest state only

This keeps the system:

-   Deterministic
-   Scalable
-   Easy to extend

------------------------------------------------------------------------

## 📈 Scaling Path

Future production upgrades could include:

-   Heartbeat / zombie detection
-   Redis-based distributed counter
-   Multi-node clustering
-   Per-user multi-device presence
-   Authentication (JWT)
-   Rate limiting
-   Observability metrics

------------------------------------------------------------------------

## 🦀 Why Rust?

Rust gives us:

-   Zero-cost concurrency
-   Memory safety
-   Predictable performance
-   No GC pauses
-   High scalability

OnlineCrab is designed as a foundation for building real-time SaaS
systems.

------------------------------------------------------------------------

## 📄 License

MIT

------------------------------------------------------------------------

## 👨‍💻 Author

Porrapat Petchdamrongskul

------------------------------------------------------------------------

## 🦀 OnlineCrab

No noise.\
No dropped state.\
Just presence.
