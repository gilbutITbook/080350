FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update
RUN apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnu
RUN apt install -y libssl-dev

WORKDIR /app

COPY ./ .

# M1 맥에서 musl을 빌드하려면 다음 ENV 변수가 설정되어야 한다
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
ENV CC_x86_64-unknown-linux-musl=x86_64-linux-gnu-gcc

RUN cargo build --target x86_64-unknown-linux-musl --release

# 최종 도커 이미지를 scratch에서 만든다
FROM scratch

WORKDIR /app

# 크기를 최소화하고자 바이너리 파일과 .env 파일을 최종 이미지에 넣는다
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-web-dev ./
COPY --from=builder /app/.env ./

# 바이너리를 실행한다
CMD ["/app/rust-web-dev"]
