FROM rust:slim-bookworm AS builder

WORKDIR /app

ENV CARGO_NET_RETRY=10
ENV RUSTUP_MAX_RETRIES=10
ENV CARGO_INCREMENTAL=0

RUN apt-get update \\
    && apt-get install -y libpq-dev libssl-dev pkg-config

COPY . .
RUN cargo build --release


FROM debian:bookworm-slim

ARG CONFIG_DIR=/local/config
ARG BIN_DIR=/local
ARG BINARY=dodopayments

RUN apt-get update \\
    && apt-get install -y ca-certificates tzdata libpq-dev curl procps

EXPOSE 3001

RUN mkdir -p ${CONFIG_DIR}

COPY --from=builder /app/target/release/${BINARY} ${BIN_DIR}/${BINARY}
COPY config/development.toml ${CONFIG_DIR}/development.toml

WORKDIR ${BIN_DIR}

CMD ["./dodopayments"]
