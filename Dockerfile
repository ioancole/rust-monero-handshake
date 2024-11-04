FROM rust:1.82

WORKDIR /usr/src/rust-monero-handshake
COPY . .
RUN cargo build

CMD ["./target/debug/rust-monero-handshake"]