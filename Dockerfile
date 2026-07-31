#FROM rustlang/rust:nightly-bookworm-slim AS development-build
#
#WORKDIR /app
#
#COPY migrations ./migrations
#
#COPY Cargo.* .
#
#COPY src src
#
#RUN cargo build
#
#FROM debian:bookworm-slim AS development-runtime
#
#WORKDIR /app
#
#COPY --from=development-build /app/target/debug/<repo-name> .
#
#CMD ["./<repo-name>"]