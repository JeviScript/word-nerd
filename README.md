# Word-nerd
  Totally overengineered vocabulary builder for personal use. Work in progress

# Structure
    .
    ├── dockerise               # Docker and script files to spin up the project locally.
    ├── common-rs               # Rust lib for shared utilities across all the rust projects here
    ├── rpc                     # Rust project with protobuf files to generate types for gRPC communication
    ├── services                # Microservices
    │   ├── account             # Rust microservice. Handles login/authentication. Database: mongodb
    │   ├── dictionary          # Rust microservice. Scans 3rd party dictionary sources. Database: mongodb
    │   ├── search              # Meilisearch powered autocompletion microservice. You guessed it, also in rust! 
    ├── web-client              # Front-end written in svelte.
    ├── web-api                 # Api gateway for the front-end written in rust and axum.
    └── README.md

## Local dev
- in dockerise folder: `./local.nu`

  It spins up all services and api in docker with virtual mount (to speed up code reload).
  
