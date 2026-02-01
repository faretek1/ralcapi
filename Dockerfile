FROM rust:1.93 AS chef
RUN cargo install cargo-chef 
WORKDIR /app

FROM chef AS planner
COPY . . 
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# build deps
RUN cargo chef cook --release --recipe-path recipe.json 
# build app
COPY . .
RUN cargo build --release --bin app

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/app /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/app" ]
