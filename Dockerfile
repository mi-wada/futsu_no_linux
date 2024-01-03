FROM rust:1.74.0

WORKDIR /app

RUN cargo install cargo-make

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build

COPY . .
