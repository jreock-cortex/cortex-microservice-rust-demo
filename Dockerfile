FROM rust:1.68 
WORKDIR /opt/addition-microservice
COPY . .

RUN cargo install --path .
CMD ["target/release/cortex-microservice-rust-demo"]
