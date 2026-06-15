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

### Local Development

For normal development and debug runs:
```bash
# Clean project
cargo clean

# Run local development server (listens on 0.0.0.0:8080)
cargo run
```
*Note: Debug symbol generation (`.pdb` file creation) is disabled for local development (`[profile.dev]`) to conserve local disk space.*

---

## Production Builds & Optimization

To compile a highly optimized production binary:
```bash
cargo build --release
```
The resulting executable will be built at `target/release/streaming-rust` (or `streaming-rust.exe` on Windows).

### Release Profile Highlights (`Cargo.toml`)
We have optimized the release profile to balance **compilation speed, executable file size, and execution speed**:
- **`lto = "thin"`**: Enables fast, parallel Link-Time Optimization (LTO) for cross-crate optimizations.
- **`codegen-units = 16`**: Restores parallel compilation to keep compilation times fast.
- **`panic = "abort"`**: Removes stack unwinding code, reducing the binary size and improving performance.
- **`strip = true`**: Strips all debug information and symbols from the compiled binary.
- **`debug = false`** & **`.cargo/config.toml`**: Explicitly disables MSVC Program Database (`.pdb`) file generation on Windows to prevent generating large unnecessary files.

*Note: All intermediate build files (like `libstreaming_rust.rlib`) are build-time requirements and should not be deployed. Only copy the final executable `streaming-rust` / `streaming-rust.exe` for production deployment.*

---

## Content Negotiation (Streaming Formats)

The `/curves/streaming` and `/productive/timeseries/streaming` endpoints dynamically adjust their output formats based on the client's `Accept` HTTP header:

1. **NDJSON (Newline Delimited JSON)**:
   * **Trigger**: Set header `Accept: application/x-ndjson` (or any string containing `ndjson`).
   * **Response Type**: `application/x-ndjson`
   * **Format**: Line-separated JSON records (`{...}\n{...}\n`).
2. **Standard JSON Streaming**:
   * **Trigger**: Default behavior (when the `Accept` header is omitted, or set to `application/json`).
   * **Response Type**: `application/json`
   * **Format**: A single valid JSON array (`[{...},{...}]`) streamed chunk-by-chunk.

---

## Docker Deployment

This repository includes a multi-stage `Dockerfile` optimized for caching dependencies and minimal production image size (~80MB).

### 1. Build the Docker Image
```bash
docker build -t streaming-rust:latest .
```

### 2. Run the Container
Pass the required configuration or database variables via environment flags:
```bash
docker run -d -p 8080:8080 \
  -e OUTBOUND_ENV=production \
  -e LOG_FORMAT=json \
  --name streaming-rust-api \
  streaming-rust:latest
```

---

## Migration Notes

Refer to [MigrationGuide.Rust.md](file:///C:/Projects/StreamingRust/MigrationGuide.Rust.md) for detailed technical mappings from the C# source of truth.
