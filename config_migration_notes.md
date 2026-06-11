# Configuration Architecture: C# to Rust

This document details the refactored configuration loading logic implemented in `StreamingRust` to achieve structure parity and values alignment with the multi-environment YAML approach of the Go project.

## 1. Summary of Config Changes

### 1.1. Environment Merging ([config.rs](file:///C:/Projects/StreamingRust/src/infrastructure/config.rs))
- **File Loading Order:** Refactored `AppConfig::load()` to load `configs/default.yaml` and then conditionally merge environment-specific overrides from `configs/<OUTBOUND_ENV>.yaml` (defaulting to `development.yaml`).
- **Config Directory:** Looks up the path to the config files using the `OUTBOUND_CONFIG_DIR` environment variable (defaulting to `"configs"`).
- **Environment Overrides:** Enabled hierarchical environment variable overrides with prefix `OUTBOUND` and `__` (double underscore) separator.

### 1.2. Struct Alignment
- **SQL DSN Layout:** Replaced the flat string values of database settings (`cmdp_sql`, `mapping_sql`, `mds_sql`, `mesap_mapping_sql`) with a nested `SqlConfig` struct that wraps a `dsn` string value.
- **Cassandra Data Centers:** Replaced the flat `hosts` array with a `data_centers` map (`HashMap<String, Vec<String>>`) representing multiple hosts grouped by data center (e.g. `lupfig` and `glattbrugg`).

### 1.3. Adaptations in Server Bootstrapping ([main.rs](file:///C:/Projects/StreamingRust/src/main.rs))
- **Database Handlers:** Passed the nested `.dsn` properties to database pool initializers:
  - `MssqlMappingResolver::new(&config.datastores.mapping_sql.dsn, &config.datastores.mds_sql.dsn)`
  - `MssqlRepository::new(&config.datastores.cmdp_sql.dsn, gate)`
- **Scylla Nodes:** Implemented logic to flatten all hosts from the `data_centers` mapping into a single collection of nodes (`cassandra_hosts`) for Scylla connection pooling, falling back to `localhost` if empty.

---

## 2. Configuration Files

1. **[configs/default.yaml](file:///C:/Projects/StreamingRust/configs/default.yaml):** Baseline defaults for local developer execution.
2. **[configs/development.yaml](file:///C:/Projects/StreamingRust/configs/development.yaml):** Development environment overrides containing target database DSN connection strings, real Cassandra clusters (`lupfig` and `glattbrugg`), base URL for authz API, and allowed users.
