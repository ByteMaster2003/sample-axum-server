# 🦀 Scalable Axum REST API

A production-ready, high-performance REST API built with the **Axum** framework and **MongoDB**. This project implements a robust 6-layer architecture designed for maintainability, type safety, and high throughput.

---

## 🏗️ Architecture Overview

The system is organised into a modular 6-layer structure to decouple business logic from infrastructure:



1.  **Layers 0-2 (Middleware):** Powered by `Tower`. Handles Logging (`tracing`), Rate Limiting, and JWT Authentication.
2.  **Layer 3 (Validation):** Custom Axum Extractors for type-safe request validation.
3.  **Layer 4 (Handlers):** Entry points for HTTP requests.
4.  **Layer 5 (Services):** Core business logic implementation.
5.  **Layer 6 (Repository):** Data access layer using `Mongoose`-style patterns with the MongoDB Rust driver.

---

## 🚀 Features

* **Asynchronous Runtime:** Built on `Tokio` for non-blocking I/O.
* **Security:** State-of-the-art JWT authentication and secure password hashing.
* **Performance:** Utilisation of `OnceLock` for efficient global configuration management.
* **Observability:** Integrated with `tracing` for structured logging and debugging.
* **Scalability:** Stateless design ready for horizontal scaling in Kubernetes.

---

## 🛠️ Tech Stack

* **Framework:** [Axum](https://github.com/tokio-rs/axum)
* **Database:** MongoDB
* **Runtime:** Tokio
* **Auth:** JWT (JSON Web Tokens)
* **Utils:** Serde, Tower-HTTP, Validator, Dotenv

---

## 🚦 Getting Started

### Prerequisites
* Rust (Latest Stable)
* MongoDB Instance (Local or Atlas)

### Setup
1. Clone the repository:
   ```bash
   git clone https://github.com/ByteMaster2003/sample-axum-server.git
   cd sample-axum-server
