FROM rust:1.93 AS builder

WORKDIR /app

# Copy Cargo.toml first for dependency caching
COPY Cargo.toml ./

# Create dummy main for dependency compilation
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove dummy code
RUN rm -rf src

# Copy actual source code
COPY src ./src

# Build the real application
RUN cargo build --release

# Runtime stage - minimal image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/vulnerable /usr/local/bin/vulnerable

# Run the application
CMD ["vulnerable"]
