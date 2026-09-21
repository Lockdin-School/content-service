FROM rustlang/rust:nightly-bookworm-slim AS development-build

WORKDIR /app

COPY migrations ./migrations

COPY Cargo.* .

COPY src src

RUN cargo build

FROM debian:bookworm-slim AS development-runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && update-ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=development-build /app/target/debug/content-service .

CMD ["./content-service"]