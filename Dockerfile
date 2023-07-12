FROM rust:1.67 as builder

WORKDIR /usr/local/app/fhevm-tfhe-cli
ADD . .
RUN cargo build --release --features tfhe/$(./scripts/get_arch.sh)

FROM debian:bullseye-slim
WORKDIR /usr/local/app
RUN apt-get install libc6 -y
COPY  --from=builder /usr/local/app/fhevm-tfhe-cli/target/release/fhevm-tfhe-cli /usr/local/bin
ENV RUST_LOG=info

CMD ["fhevm-tfhe-cli"]
