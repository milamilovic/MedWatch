FROM rust:1.93 as builder

WORKDIR /app

COPY Cargo.toml ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src ./src

RUN touch src/main.rs && cargo build --release

# Runtime image - use Ubuntu for newer GLIBC
FROM ubuntu:24.04

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/sns-secure /app/sns-secure

# Use ubuntu user (UID 1000 already exists in Ubuntu 24.04)
RUN chown -R ubuntu:ubuntu /app

USER ubuntu

EXPOSE 8080

CMD ["/app/sns-secure"]