FROM rust:latest

WORKDIR /api

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo 'fn main() { println!("Hello, World!"); }' > src/main.rs

RUN cargo fetch

COPY ./src ./src

RUN cargo build --release

CMD ["./target/release/WIK-DPS-TP01"]