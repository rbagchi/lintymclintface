# Stage 1: Builder
FROM rust:1.82-slim-bookworm AS builder

WORKDIR /app

# Install system dependencies required for openssl-sys (used by reqwest/actix-web)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo.toml and Cargo.lock first to leverage Docker cache
COPY Cargo.toml Cargo.lock ./ 

# This step will download and cache dependencies
# We create a dummy src/main.rs to make cargo build happy
RUN mkdir src && echo "fn main() {}\n" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy the actual source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Stage 2: Runner
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/lintymclintface .

# Expose the port the service runs on
EXPOSE 8080

# Command to run the application in web service mode
CMD ["./lintymclintface", "--service"]
