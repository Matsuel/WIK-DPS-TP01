FROM rust:latest

WORKDIR /api

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo 'fn main() { println!("Hello, World!"); }' > src/main.rs

RUN cargo fetch

COPY ./src ./src

RUN cargo build --release

CMD ["./target/release/WIK-DPS-TP01"]

# Faire 2 build un avec les dependances et sans code source en créant un projet rust vide
# et ensuite on copie le code source dans le conteneur et le build sera plus rapide
# Supprimer les fichiers Cargo.lock et Cargo.toml pour ne pas avoir de conflit de version