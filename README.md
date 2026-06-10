# StreamingRust: Transactional Outbound API

A high-performance Rust migration of the Transactional Outbound API, originally in C#.

## Architecture

This project follows **Hexagonal Architecture** (Ports & Adapters) and **SOLID** principles.

- **src/domain**: Business entities and value objects (Framework-agnostic).
- **src/application**: Core business logic, orchestration, and interface definitions (Ports).
- **src/infrastructure**: Concrete implementations of external services (Adapters: SQL, Cassandra, HTTP).
- **src/apperr**: Centralized error handling following the Problem Details for HTTP APIs (RFC 7807) standard.

## Tech Stack

- **Runtime**: Tokio
- **Web**: Axum
- **Serialization**: Serde (with PascalCase support for C# parity)
- **Databases**: Tiberius (MSSQL), Scylla (Cassandra), Fred (Redis)
- **Parsing**: ANTLR4 (via antlr-rust)

## Getting Started

### Prerequisites

- Rust 1.75+
- Java (only for ANTLR code generation)

### Build

`ash
cargo build
`

### Run

`ash
cargo run
`

## Migration Notes

Refer to MigrationGuide.Rust.md for detailed technical mappings from the C# source of truth.
