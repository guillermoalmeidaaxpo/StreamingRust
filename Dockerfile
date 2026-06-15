# ==========================================
# Stage 1: Builder
# ==========================================
FROM rust:1.80-slim-bookworm AS builder

# Install system dependencies required for compilation
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy Cargo configuration files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create dummy sources and build script to pre-compile and cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "" > src/lib.rs && \
    echo "fn main() {}" > build.rs

# Build dependencies only (this layer will be cached unless Cargo.toml/lock changes)
RUN cargo build --release

# Copy the actual project files
COPY build.rs ./build.rs
COPY src/ ./src/
COPY grammar/ ./grammar/

# Touch the source files to ensure the compiler rebuilds the real code instead of the dummy code
RUN touch src/main.rs src/lib.rs build.rs

# Build the optimized production binary
RUN cargo build --release

# ==========================================
# Stage 2: Runtime Environment
# ==========================================
FROM debian:bookworm-slim

# Install system runtime dependencies (like OpenSSL and CA certs for SSL connections)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built release binary from the builder stage
COPY --from=builder /usr/src/app/target/release/streaming-rust /app/streaming-rust

# Copy configuration files
COPY configs/ /app/configs/

# Expose the API port
EXPOSE 8080

# Configure production environment defaults
ENV OUTBOUND_ENV=production
ENV LOG_FORMAT=json

# Run the service
CMD ["/app/streaming-rust"]
