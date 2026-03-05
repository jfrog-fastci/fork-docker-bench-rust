FROM rust:1.80

RUN apt-get update && apt-get install -y vim less man-db wget telnet curl net-tools iputils-ping htop dnsutils strace pkg-config libssl-dev

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/docker-bench-rust"]
