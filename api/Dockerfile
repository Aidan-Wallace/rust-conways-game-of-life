FROM rust:1.87 AS build-env

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY api ./api
COPY console ./console
COPY game-of-life ./game-of-life

RUN cargo build --release --package api

FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY --from=build-env /app/target/release/api .
COPY api/templates ./templates
COPY api/static ./static
COPY presets.json ./presets.json

ENV HOST="0.0.0.0"
ENV DEFAULT_PORT="80"
ENV HTML_FILE_FP="/app/templates/index.html"
ENV STATIC_DIR_FP="/app/static"
ENV PRESETS_FILE_FP="/app/presets.json"

EXPOSE ${DEFAULT_PORT}
ENTRYPOINT ["./api"]
