FROM rust:1.63-buster as builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/dice-api /usr/bin

EXPOSE 8000
HEALTHCHECK --interval=10s --timeout=10s --start-period=5s \
            CMD curl --fail http://localhost:8000/health || exit 1
ENTRYPOINT ["dice-api"]