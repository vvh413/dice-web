FROM rust:1.63-slim as builder 
RUN apt-get update && apt-get install -y musl-tools curl
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine
RUN apk --no-cache add curl
COPY --from=builder ./target/x86_64-unknown-linux-musl/release/dice-api /

EXPOSE 8000
HEALTHCHECK --interval=10s --timeout=10s \
            CMD curl -f -s http://localhost:8000/health || exit 1
ENTRYPOINT ["/dice-api"]
