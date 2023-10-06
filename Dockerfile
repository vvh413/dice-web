FROM clux/muslrust:stable AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY services/backend /app/backend
WORKDIR /app/backend
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS rust-builder
COPY --from=planner /app/backend/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY services/backend /app/backend
WORKDIR /app/backend
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM node:lts as node-builder
COPY services/frontend /app/frontend
WORKDIR /app/frontend
RUN yarn install && yarn build

FROM alpine
RUN apk --no-cache add curl
WORKDIR /app
COPY --from=rust-builder /app/backend/target/x86_64-unknown-linux-musl/release/dice-api /app
COPY --from=node-builder /app/frontend/dist /app/frontend

EXPOSE 8000
HEALTHCHECK --interval=10s --timeout=10s \
  CMD curl -f -s http://localhost:8000/health || exit 1
ENTRYPOINT ["/app/dice-api"]
