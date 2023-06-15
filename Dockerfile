FROM rust:1.67 as builder

WORKDIR /usr/local/app/zbc-fhe-tool
ADD . .
RUN cargo build --release --features tfhe/$(./scripts/get_arch.sh)

FROM debian:bullseye-slim
WORKDIR /usr/local/app
RUN apt-get install libc6 -y
COPY  --from=builder /usr/local/app/zbc-fhe-tool/target/release/zbc-fhe-tool /usr/local/bin
ENV RUST_LOG=info

CMD ["zbc-fhe-tool"]
