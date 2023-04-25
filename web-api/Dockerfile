FROM rust:latest as builder
RUN apt update && apt upgrade -y && apt install -y protobuf-compiler libprotobuf-dev
# cashes recompilation of crates
workdir /word-nerd/web-api
RUN echo "fn main() {}" > dummy.rs
copy ./web-api/Cargo.toml . 
copy ./rpc ../rpc
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY ./web-api .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/web-api /usr/local/bin/web-api
CMD ["web-api"]

