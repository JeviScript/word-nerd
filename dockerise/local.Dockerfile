from rust:latest as rust_base
run cargo install cargo-watch
run apt update && apt upgrade -y && apt install -y protobuf-compiler libprotobuf-dev

from rust_base as rust_ms
run cargo install grpc_health_probe

from node:18.15.0 as node_base



