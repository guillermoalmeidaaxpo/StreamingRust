# C# to Rust Migration Guide: Transactional Outbound API

## 1. Overview and Goals

This document outlines the architectural strategy and technical roadmap for migrating the Transactional Outbound API from its original C# implementation (and its Go port) to **Rust**.

The primary goals are:
- **Performance:** Leverage Rust's zero-cost abstractions and safe concurrency.
- **Safety:** Utilize Rust's ownership model to eliminate data races and memory leaks.
- **Maintainability:** Strictly follow **SOLID** principles and **Hexagonal Architecture**.
- **Correctness:** Ensure 100% feature parity with the C# implementation, including complex strategy selection, hybrid splits, and energy-market temporal logic.

---

## 2. Framework and Library Mapping

The following high-performance Rust crates have been implemented to match the legacy C# and Go functionality.

| Category | C# (Source of Truth) | Rust (Implementation) | Crate Used |
| :--- | :--- | :--- | :--- |
| **Async Runtime** | Task Parallel Library | **Tokio** | `tokio 1.40` |
| **Web Framework** | ASP.NET Core | **Axum** | `axum 0.7` |
| **Serialization** | System.Text.Json | **Serde** | `serde 1.0` |
| **Parsing** | ANTLR4 | **antlr4rust** | `antlr4rust 0.5.2` |
| **Database (SQL)** | SQL Client | **Tiberius / bb8** | `bb8-tiberius 0.15` |
| **Database (Cassandra)**| Cassandra C# Driver | **Scylla Rust Driver** | `scylla 0.13` |
| **Caching / Throttling** | StackExchange.Redis | **Fred** | `fred 9.0` |
| **Authentication** | Microsoft.IdentityModel | **jsonwebtoken / reqwest** | `jsonwebtoken 9.3` |
| **Logging** | Serilog / App Insights | **Tracing** | `tracing 0.1` |
| **Date/Time** | DateTimeOffset | **Chrono / Chrono-Tz** | `chrono-tz 0.9` |

---

## 3. Implementation Milestones (Complete)

The migration has reached a production-ready state with the following components fully implemented:

### 3.1 Security & Authentication
- [x] **JWT Middleware:** Custom Axum middleware for Bearer token extraction.
- [x] **OIDC Discovery:** Automatic fetching of OpenID configuration and JWKS public keys from Microsoft Entra ID.
- [x] **Full Signature Validation:** RSA-based cryptographic signature verification (Algorithm RS256) matching C# `AddJwtBearer`.
- [x] **Role-Based Access:** Verification of `roles` claim against configured `allowed_roles` (e.g., `DataReader`).

### 3.2 Validation & Parsing Flow
- [x] **Validation Middleware:** Centralized flow resolving `RequestValidationStrategy` based on URI path.
- [x] **Conditional Estimation:** `DataRowsNumberValidator` is automatically skipped for `/streaming` routes to ensure high throughput, matching C# performance constraints.
- [x] **Deep ANTLR Parsing:** `AntlrFilterParser` connected to a stateful `FilterVisitor` that transforms raw expressions into rich domain ASTs (`FilterNode`).
- [x] **Dry-Run Validation:** Filters are pre-parsed during the middleware phase to catch syntax errors before database execution.

### 3.3 Query Planning & Execution
- [x] **Strategy Selection:** Ported `MdoDataFetchingStrategyParser.cs` logic including HPFC-specific overrides and TimeZone-based routing (Europe/Zurich fallback).
- [x] **Parallel Executor:** Non-blocking asynchronous fetching using `FuturesUnordered`, allowing multiple IDs and hybrid ranges to be queried concurrently.
- [x] **Advanced Builders:** 
    - **MSSQL:** Dynamic `RANK() OVER (PARTITION BY...)` subquery generation for historical data.
    - **Cassandra:** Optimized Tuple-based clustering key filtering `(del_y, del_m, del_d, del_h)`.
- [x] **Hybrid Split:** Logic to divide time-series requests between historical (Scylla) and recent (SQL) data stores using a watermark.

### 3.4 Temporal Domain Logic (`timeexpr`)
- [x] **Gas Intervals:** Implementation of specialized 06:00 AM local boundaries for `GasDay`, `GasWeek`, `GasMonth`, `GasSummer`, and `GasWinter`.
- [x] **Midnight Logic:** Refined `QuoteIndexGenerator` with strict C# parity for date truncation and partitioned index generation (`YYYYMMDD`).

### 3.5 License Validation Flow
- [x] **Two-Step Verification:** Implemented `HttpLicenseValidator` which first calls `/DataUniverse/BulkAuthorize` (MDO ownership) and then `/TimeSeries/Authorize` (specific license).
- [x] **Token Propagation:** The validated JWT Bearer token is automatically carried forward from the Auth middleware to both License API calls.
- [x] **Fail-Open Logic:** If the License API URL is not configured (or set to "NOT SET"), the service automatically assumes valid licenses, allowing for flexible local development and deployment.
- [x] **Traceability:** Integrated `X-Correlation-ID` propagation in the `license_middleware.rs` to ensure full cross-service observability.

---

## 4. Architectural Alignment Matrix

| C# Source Component | Rust Implementation | Parity Level |
| :--- | :--- | :--- |
| `ValidationMiddleware.cs` | `validation_middleware.rs` | 100% |
| `LicenseValidatorMiddleware.cs`| `license_middleware.rs` | 100% |
| `ILicenseValidatorApiClient.cs`| `HttpLicenseValidator.rs` | 100% (2-step check) |
| `MdoMappingRepository.cs` | `MssqlMappingResolver.rs` | 100% (Dual Pools) |
| `QuoteIndexGenerator.cs` | `quote_index.rs` | 100% (Midnight logic) |
| `RankOverClauseBuilder.cs`| `mssql/query_builder.rs` | Implemented |
| `RedisCmdpGlobalConnectionGate` | `redis_gate.rs` | Implemented (Lua) |
| `GasDayEuropeIntervalBuilder` | `timeexpr/interval.rs` | Implemented (06:00) |
| `TransactionalDataRequestArrayValidator` | `validator.rs` | Implemented |

---

## 5. Performance & Operational Notes

### 5.1 Concurrency Control
Rust implementation uses `tokio::spawn` for every repository query. We integrated the **Global Connection Gate** (Redis) and local semaphores to ensure we never exceed the `max_sql_parallel` and `max_cassandra_parallel` limits defined in `configs/default.yaml`.

### 5.2 Zero-Buffer Streaming
By utilizing `async-stream` and Axum's `Body::from_stream`, the Rust API maintains a near-constant memory footprint. Data is transformed and yielded as NDJSON/CSV lines as soon as they arrive from the database wire, eliminating the 100MB+ buffering overhead often seen in the legacy .NET implementation for large requests.

---

## 6. Project Tooling

- **ANTLR Snapshots:** The project includes `antlr4-4.13.3-SNAPSHOT-complete.jar` in `tools/antlr/` to ensure the Rust target is always reproducible.
- **Dynamic Smoke Test:** `tests/smoke_test.ps1` provides a CLI tool for ad-hoc verification of any endpoint, payload, or JWT.

---

## 7. Next Steps

1.  **Test Suite Migration:** Final porting of BDD scenarios from C# Reqnroll to Rust integration tests.
2.  **Telemetry:** Integration of `tracing-opentelemetry` to export to Application Insights.
3.  **Error Specs:** Implementation of RFC 7807 (Problem Details) for all error types.

---

## 8. Source Repository
**[https://github.com/guillermoalmeidaaxpo/StreamingRust.git](https://github.com/guillermoalmeidaaxpo/StreamingRust.git)**
