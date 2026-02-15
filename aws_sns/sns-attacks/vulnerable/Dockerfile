FROM rust:1.93 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Create dummy main to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy real source
COPY src ./src

# Build for release
RUN touch src/main.rs && cargo build --release

# Runtime image - use Ubuntu for newer GLIBC
FROM ubuntu:24.04

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/sns-vulnerable /app/sns-vulnerable

EXPOSE 8080

CMD ["/app/sns-vulnerable"]