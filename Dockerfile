FROM rust:1.79 AS build-env

WORKDIR /app

COPY Cargo.* ./
COPY src ./src

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=build-env /app/target/release/game-of-life /

ENTRYPOINT ["./game-of-life"]
