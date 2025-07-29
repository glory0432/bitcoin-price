FROM rust:1.88.0 as builder

WORKDIR /backend

COPY Cargo.toml .
COPY Cargo.lock .

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

RUN rm -f src/main.rs
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /backend
COPY --from=builder /backend/target/release/backend .

CMD [ "./backend" ]
