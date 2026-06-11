# Shape + Aggregation Migration Report: C# to Rust

This document details the successful translation of the **Shape filter + Aggregation** features from the legacy C# Transactional Outbound API to the **Rust** implementation (`StreamingRust`).

## 1. Summary of Changes

To maintain full parity with the latest C# release (which enabled combining Shape filters with Aggregations and added strict validation for curve datastores), we implemented the following changes across the Rust codebase:

### 1.1. Domain Layer (`src/domain/request.rs`)
- **DTO Definitions:** Added strongly-typed `Shape` and `TimeRange` structs matching the C# JSON contract.
- **Custom Deserializer:** Implemented a custom Serde `Deserialize` and `Serialize` implementation for `TimeRange` to handle the `.NET` format `TimeSpan(T##:##:##, T##:##:##)`.
- **Normalization Engine:** Created `NormalizedShape` and `NormalizedTimeSpan` structs. Added a `normalize()` method on the raw `Shape` struct to convert string abbreviations (e.g., `Jan` or `Mon`) into 1-based integers (`1` to `12` and `1` to `7`) and times into duration offsets.

### 1.2. Application Layer (`src/application/`)
- **Command Extension ([ports.rs](file:///C:/Projects/StreamingRust/src/application/ports.rs)):** Added the `shape: Option<NormalizedShape>` field to the core execution `Command`.
- **Data-Store Routing ([planner.rs](file:///C:/Projects/StreamingRust/src/application/planner.rs)):**
  - Populated the normalized shape on the built command.
  - Enforced the restriction that Shape filters cannot be used with non-CMDP curve identifiers (Hyperscale mapping) by throwing an error with the exact legacy message: `"The Shape filter cannot be used with this Identifier. Data is not hosted in CMDP"`.
- **Validation Suite ([validator.rs](file:///C:/Projects/StreamingRust/src/application/validator.rs)):**
  - **Month/Day Check:** Verifies correct 3-letter month and day abbreviations and detects duplicates.
  - **Time Range Check:** Ensures start time is earlier than the end time (interpreting `T00:00:00` end time as the end of the day).
  - **Uniqueness & Overlap Check:** Prevents duplicate time ranges and overlapping intervals (using standard segment overlap algorithm).

### 1.3. Infrastructure Layer (`src/infrastructure/`)
- **Category Enforcement ([validation_middleware.rs](file:///C:/Projects/StreamingRust/src/infrastructure/http/validation_middleware.rs)):**
  - Refactored the Axum validation middleware to return the standard RFC 7807 `AppError` format rather than a generic status code.
  - Blocked non-Curve endpoints from accepting Shape filters, returning an error message matching the C# contract: `"Shape filter is only supported for data category 'Curve'."`.
- **SQL Query Generation ([query_builder.rs](file:///C:/Projects/StreamingRust/src/infrastructure/mssql/query_builder.rs)):**
  - **Delivery Start Expression:** Resolves the `DeliveryStart` column from mappings and applies the `AT TIME ZONE` SQL expression using a mapped SQL Server Windows timezone name.
  - **Month Clause:** Appends `DATEPART(MONTH, [col]) IN (@month0, ...)` filters.
  - **Day Clause:** Appends `((DATEDIFF(DAY, '19000101', [col]) % 7) + 1) IN (@day0, ...)` filters.
  - **Time-of-Day Clause:** Generates `CAST([col] AS time) >= @t_start AND CAST([col] AS time) < @t_end` ranges. If the end time is midnight (`T00:00:00`), it correctly converts it into a lower-bound only predicate `[col] >= @t_start` to represent the end-of-day.

### 1.4. Test Suite (`tests/logic_tests.rs`)
- Added comprehensive unit tests:
  - `test_shape_normalization` for month/day mapping and duration conversions.
  - `test_shape_validation_success` for valid Shape requests.
  - `test_shape_validation_failures` covering invalid months, duplicate days, invalid ranges, and overlapping periods.

---

## 2. Code Mappings Reference

| C# Source Component | Rust Destination | Status |
| :--- | :--- | :--- |
| `Shape.cs` | `request.rs` | 100% Parity |
| `TimeRange.cs` | `request.rs` | 100% Parity |
| `TimeRangeJsonConverter.cs` | `request.rs` (custom impls) | 100% Parity |
| `NormalizedShape.cs` | `request.rs` | 100% Parity |
| `ShapeNormalizer.cs` | `request.rs` (normalize method) | 100% Parity |
| `ShapeValidator.cs` | `validator.rs` (validate_shape) | 100% Parity |
| `SqlShapePredicateBuilder.cs` | `query_builder.rs` | 100% Parity |
| `SqlTimeZoneHelper.cs` | `query_builder.rs` (timezone helpers) | 100% Parity |
