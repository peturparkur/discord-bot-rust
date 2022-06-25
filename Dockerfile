ARG BINARY_NAME = discord-bot

FROM rust:1.61 as builder
RUN USER=root cargo new --bin $BINARY_NAME-build
WORKDIR /$BINARY_NAME-build

# copy dependencies and build for caching
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

# remove files since they're cached
RUN rm src/*.rs

# copy read files and build
COPY ./src ./src
RUN rm ./target/release/deps/$BINARY_NAME*
RUN cargo build --release

FROM debian:buster-slim
# install runtime dependencies
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

# Instal SSL certificate
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

COPY --from=builder /$BINARY_NAME-build/target/release/$BINARY_NAME .
CMD ["discord-bot"]