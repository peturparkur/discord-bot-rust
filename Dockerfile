ARG BINARY_NAME=discord-bot
ARG TMP_NAME=discord-bot-build

FROM rust:1.61 as builder
ARG BINARY_NAME
ARG TMP_NAME

RUN USER=root cargo new --bin ${TMP_NAME}
WORKDIR /${TMP_NAME}

# copy dependencies and build for caching
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

# remove files since they're cached
RUN rm src/*.rs

# copy read files and build
COPY ./src ./src
RUN ["/bin/bash", "-c", "rm ./target/release/deps/${BINARY_NAME//-/_}*"]
RUN cargo build --release

FROM debian:buster-slim
ARG BINARY_NAME
ARG TMP_NAME

# install runtime dependencies
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

# Instal SSL certificate
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

COPY --from=builder /${TMP_NAME}/target/release/${BINARY_NAME} .
# discord-bot
CMD ["./discord-bot"]