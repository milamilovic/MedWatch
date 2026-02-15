FROM rust:1.93 AS builder

WORKDIR /app

# Copy Cargo.toml first for dependency caching
COPY Cargo.toml ./

# Create dummy main for dependency compilation
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# IMPORTANT: Clean up ALL build artifacts to force rebuild
RUN rm -rf src target/release/deps/mitigated* target/release/mitigated target/release/.fingerprint/mitigated*

# Copy actual source code
COPY src ./src

# Force timestamp update
RUN touch src/main.rs

# Build the real application - this MUST recompile
RUN cargo build --release

# Verify the binary exists
RUN ls -lh /app/target/release/mitigated

# Runtime stage - minimal image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/mitigated /usr/local/bin/mitigated

# Run the application
CMD ["mitigated"]
