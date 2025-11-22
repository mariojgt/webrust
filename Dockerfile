# Build stage
FROM rust:1.82-slim-bookworm as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev default-libmysqlclient-dev

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 default-libmysqlclient-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary and assets
COPY --from=builder /app/target/release/webrust /app/webrust
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/public /app/public
COPY --from=builder /app/.env.example /app/.env

# Expose port
EXPOSE 8000

# Run the application
CMD ["./webrust", "rune", "serve", "--host", "0.0.0.0", "--port", "8000"]
