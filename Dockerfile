FROM rustlang/rust:nightly-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
# Copy only the rust-toolchain.toml first to set up the toolchain
COPY rust-toolchain.toml .
# Set RUSTUP_TOOLCHAIN to avoid cross-device link errors from rustup self-updates
ENV RUSTUP_TOOLCHAIN=nightly
ENV RUSTUP_UPDATE_ROOT=/tmp/rustup-update
COPY . .
ENV SQLX_OFFLINE=true
# Remove rust-toolchain.toml to prevent rustup from trying to update the toolchain in Docker
RUN rm -f rust-toolchain.toml app/rust-toolchain.toml
RUN cargo +nightly build --release --features ssr --bin todomvc --manifest-path app/Cargo.toml

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/todomvc /usr/local/bin/todomvc
COPY --from=builder /app/app/migrations /migrations
ENV DATABASE_URL=sqlite:/data/todos.db
ENV PORT=8080
EXPOSE 8080
RUN mkdir -p /data
CMD ["todomvc"]
