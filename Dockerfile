FROM rust:1.63-buster as builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/dice-api /usr/bin

EXPOSE 8000
ENTRYPOINT ["dice-api"]