FROM rust:1.74.0

WORKDIR /app

RUN cargo install cargo-make

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && \
    mkdir -p src/05/cat && echo 'fn main() {}' > src/05/cat/main.rs
RUN cargo build

COPY . .
