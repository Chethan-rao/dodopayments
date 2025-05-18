# Use a slim Rust image as the base for the builder stage.
FROM rust:slim-bookworm AS builder

# Set the working directory inside the container.
WORKDIR /app

# Set environment variables for Cargo.
ENV CARGO_NET_RETRY=10
ENV RUSTUP_MAX_RETRIES=10
ENV CARGO_INCREMENTAL=0

# Install necessary dependencies.
RUN apt-get update \
    && apt-get install -y libpq-dev libssl-dev pkg-config

# Copy the project files into the container.
COPY . .
# Build the application in release mode.
RUN cargo build --release

# Use a slim Debian image as the base for the final stage.
FROM debian:bookworm-slim

# Define arguments for configuration and binary directories.
ARG CONFIG_DIR=/local/config
ARG BIN_DIR=/local
ARG BINARY=dodopayments

# Install necessary dependencies.
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev curl procps

# Expose port 3001.
EXPOSE 3001

# Create the configuration directory.
RUN mkdir -p ${CONFIG_DIR}

# Copy the built binary and configuration file into the container.
COPY --from=builder /app/target/release/${BINARY} ${BIN_DIR}/${BINARY}
COPY config/development.toml ${CONFIG_DIR}/development.toml

# Set the working directory for the final stage.
WORKDIR ${BIN_DIR}

# Command to run the application.
CMD ["./dodopayments"]
