# ---- BUILD ----
FROM rustlang/rust:nightly AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# ---- RUNTIME ----
FROM debian:12-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/rust_iss /app/rust_iss

EXPOSE 3000
CMD ["/app/rust_iss"]
