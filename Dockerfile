FROM rust:1.93
WORKDIR /app

COPY cargo.lock cargo.toml ./
RUN cargo update

COPY . .
CMD ["cargo", "run"]
