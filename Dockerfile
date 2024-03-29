FROM rust:latest

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

COPY . .

RUN rustup default nightly
RUN cargo build

CMD ["./target/debug/rusting"]
