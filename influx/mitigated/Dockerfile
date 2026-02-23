FROM rust:1.83-alpine as builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY Cargo.toml .
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release && rm -rf src
COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM alpine:3.19
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/mitigated-app /usr/local/bin/
CMD ["mitigated-app"]
