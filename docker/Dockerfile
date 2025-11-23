# --- Stage 1: Frontend Build ---
FROM node:20-slim AS frontend

WORKDIR /app

# Copy frontend configuration
COPY package.json package-lock.json* ./
COPY vite.config.js tailwind.config.js postcss.config.js ./

# Install dependencies
RUN npm install

# Copy frontend source
COPY resources ./resources
COPY templates ./templates
COPY public ./public

# Build assets
RUN npm run build

# --- Stage 2: Backend Build ---
FROM rust:slim-bookworm AS backend

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev default-libmysqlclient-dev

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies (caching layer)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY src ./src
COPY templates ./templates

# Update the modification time of the source files to ensure Cargo rebuilds the binary
# (Fixes issue where dummy build artifact is newer than copied source)
RUN touch src/main.rs

# Build the application
RUN cargo build --release

# --- Stage 3: Runtime ---
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    default-libmysqlclient-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from backend builder
COPY --from=backend /app/target/release/webrust /app/webrust

# Copy built assets from frontend builder
COPY --from=frontend /app/public/build /app/public/build

# Copy other static assets and templates
COPY --from=backend /app/templates /app/templates
COPY public /app/public
COPY migrations /app/migrations

# Copy environment file (ensure you have .env or handle this via docker-compose)
COPY .env.example /app/.env

# Expose port
EXPOSE 8000

# Run the application
CMD ["./webrust", "rune", "serve", "--host", "0.0.0.0", "--port", "8000"]
