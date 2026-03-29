FROM rust:1.85-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --features ssr --bin todomvc --manifest-path app/Cargo.toml

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/todomvc /usr/local/bin/todomvc
COPY --from=builder /app/app/migrations /migrations
ENV DATABASE_URL=sqlite:/data/todos.db
ENV PORT=8080
EXPOSE 8080
RUN mkdir -p /data
CMD ["todomvc"]
