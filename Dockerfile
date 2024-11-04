FROM rust:1.82

WORKDIR /usr/src/rust_monero_handshake

COPY . .

RUN cargo build --release

# List the contents of the target/release directory for debugging
RUN pwd

RUN chmod +x ./target/release/rust_monero_handshake
CMD ["./target/release/rust_monero_handshake"]