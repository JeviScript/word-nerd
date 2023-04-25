FROM rust:latest as builder
RUN apt update && apt upgrade -y && apt install -y protobuf-compiler libprotobuf-dev
# cashes recompilation of crates
workdir /word-nerd/services/account
RUN echo "fn main() {}" > dummy.rs
copy ./services/account/Cargo.toml . 
copy ./rpc ../../rpc
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY ./services/account .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt update && apt upgrade -y && apt install openssl && apt install ca-certificates
COPY --from=builder /usr/local/cargo/bin/account-ms /usr/local/bin/account-ms
CMD ["account-ms"]
