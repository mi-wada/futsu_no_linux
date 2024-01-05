FROM rust:1.74.0

WORKDIR /app

RUN cargo install cargo-make

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && \
    mkdir -p src/05/cat && echo 'fn main() {}' > src/05/cat/main.rs && \
    mkdir -p src/05/wc && echo 'fn main() {}' > src/05/wc/main.rs && \
    mkdir -p src/06/cat_no_libc && echo 'fn main() {}' > src/06/cat_no_libc/main.rs && \
    mkdir -p src/07/head && echo 'fn main() {}' > src/07/head/main.rs && \
    mkdir -p src/07/tail && echo 'fn main() {}' > src/07/tail/main.rs && \
    mkdir -p src/08/grep && echo 'fn main() {}' > src/08/grep/main.rs && \
    mkdir -p src/10/ls && echo 'fn main() {}' > src/10/ls/main.rs && \
    mkdir -p src/10/mkdir && echo 'fn main() {}' > src/10/mkdir/main.rs && \
    mkdir -p src/10/rmdir && echo 'fn main() {}' > src/10/rmdir/main.rs
RUN cargo build

COPY . .
