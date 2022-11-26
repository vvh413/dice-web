FROM rust:1.63-slim as rust-builder
RUN apt-get update && apt-get install -y musl-tools curl
RUN rustup target add x86_64-unknown-linux-musl
COPY services/backend ./backend
WORKDIR ./backend
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM node:lts as node-builder
COPY services/frontend ./frontend
WORKDIR ./frontend
RUN yarn install
RUN yarn build

FROM alpine
RUN apk --no-cache add curl
COPY --from=rust-builder ./backend/target/x86_64-unknown-linux-musl/release/dice-api /
COPY --from=node-builder ./frontend/dist /

EXPOSE 8000
HEALTHCHECK --interval=10s --timeout=10s \
            CMD curl -f -s http://localhost:8000/health || exit 1
ENTRYPOINT ["/dice-api"]
