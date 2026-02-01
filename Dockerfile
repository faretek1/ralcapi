FROM rust:1.93
WORKDIR /app

COPY Cargo.lock Cargo.toml ./
RUN cargo update

COPY . .
CMD ["cargo", "run"]
