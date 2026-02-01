FROM rust:1.93
WORKDIR /app

COPY Cargo.lock Cargo.toml ./
RUN cargo fetch --locked

COPY . .
CMD ["cargo", "run"]
