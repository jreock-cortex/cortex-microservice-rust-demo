FROM rust:1.75 
WORKDIR /opt/addition-microservice
COPY . .

RUN cargo install --path .
CMD ["target/release/cortex-microservice-rust-demo"]
