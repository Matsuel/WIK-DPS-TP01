FROM rust:alpine3.21 as builder

WORKDIR /api

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo 'fn main() { println!("Hello, World!"); }' > src/main.rs

RUN cargo fetch

COPY ./src ./src

RUN cargo build --release && rm -rf src target/debug Cargo.lock Cargo.toml

FROM scratch

COPY --from=builder /api/target/release/WIK-DPS-TP01 /api

CMD [ "/api" ]


# Build stage
# From scratch stage

# -> Analyse du Dockerfile 2.md