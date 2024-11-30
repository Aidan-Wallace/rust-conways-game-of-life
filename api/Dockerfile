FROM rust:1.79 AS build-env

WORKDIR /app

COPY Cargo.* .
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=build-env /app/target/release/api /

ENV HOST="localhost"
ENV DEFAULT_PORT=80

EXPOSE ${DEFAULT_PORT}
ENTRYPOINT ["./api"]
