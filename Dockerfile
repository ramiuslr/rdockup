FROM rust:1.87 as builder
WORKDIR /build
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release

FROM rust:1.87-slim-bookworm
WORKDIR /app
COPY --from=builder /build/target/release/rdockup .
CMD ["./rdockup"]
