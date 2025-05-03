# Weather Tracking System – Assignment 12

## 📋 Overview
This assignment focuses on building a service layer for the Weather Tracking System. The goal was to expose a RESTful API for CRUD operations on weather reports and document it using OpenAPI 3.0. It builds upon the repository and domain layers completed in Assignment 11.

---

## 🧱 Architecture

- **Domain Layer**: Defines core models like `WeatherReport`.
- **Repository Layer**: Includes a generic `Repository` interface and an in-memory implementation.
- **Service Layer**: Encapsulates business logic; decoupled from storage details via repository interfaces.
- **API Layer**: REST endpoints for interacting with weather data.
- **OpenAPI Spec**: Full YAML-based documentation of the exposed REST API.

---

## 🧩 Features

- 📄 **CRUD** operations for Weather Reports.
- 📦 **In-memory storage** via `HashMap`.
- 🔌 **Service abstraction** for business logic.
- 🌐 **REST API** built using `axum`.
- 📚 **OpenAPI 3.0** specification for API documentation.
- ✅ **Unit tests** for API functionality using `reqwest` and `tokio`.

---

## 📂 Directory Structure

rc/
├── api/ # REST API routes and handlers
├── domain/ # Domain entities (WeatherReport)
├── repositories/
│ ├── mod.rs # Generic interfaces
│ └── inmemory/ # In-memory implementations
├── services/ # Business logic layer
├── factories/ # Repository factory for storage decoupling
├── main.rs # App startup
tests/ # API integration tests
openapi.yaml # OpenAPI 3.0 Spec


README.md


---

## 🔁 API Endpoints

| Method | Path                | Description              |
|--------|---------------------|--------------------------|
| GET    | `/weather`          | List all reports         |
| POST   | `/weather`          | Create a new report      |
| GET    | `/weather/{id}`     | Fetch a report by ID     |

All endpoints accept and return JSON payloads. Example payload:



Testing


Run the tests:

cargo test


The test suite verifies:

POST + GET flow.

404 error for unknown IDs.

API returns correct content type and structure.


OpenAPI Documentation

The API is documented using OpenAPI 3.0 (openapi.yaml). It can be visualized using:

Swagger Editor

ReDoc


🛠 Technologies

Rust

Axum

Serde

Tokio

OpenAPI 3.0

