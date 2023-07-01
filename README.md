# Word-nerd
  Totally overengineered vocabulary builder for personal use. Work in progress

# Structure
    .
    ├── dockerise               # Docker and script files to spin up the project locally.
    ├── rpc                     # Rust project with protobuf files to generate types for gRPC communication
    ├── services                # Microservices
    │   ├── account             # Rust microservise. Handles login/authentication. Database: mongodb
    │   ├── dictionary          # Rust microservice. Scans 3rd party dictionary sources. Database: mongodb
    ├── web-client              # Front-end written in svelte.
    ├── web-api                 # Api gateway for the front-end written in rust and axum.
    └── README.md

## Local dev
- in dockerise folder: `./local.nu`

  It spins up all services and api in docker with virtual mount (to speed up code reload).
  
