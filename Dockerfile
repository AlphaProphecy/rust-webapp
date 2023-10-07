FROM rust:1.73 AS builder

WORKDIR /usr/src/rust-webapp

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir ./src &&\
    echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/\
    --mount=type=cache,target=/usr/src/rust-webapp/target
RUN  cargo install --path .
RUN rm -rf ./src

COPY . .

# Touch main.rs to invalidate cache
RUN touch ./src/main.rs
RUN cargo install --path .

FROM debian:bookworm-slim as runtime
WORKDIR /var/www/rust-webapp
RUN apt-get update &&\
    apt-get install -y &&\
    apt-get -y install libmariadb-dev &&\
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust-webapp /usr/local/bin/rust-webapp

COPY --link ./Rocket.toml ./Rocket.toml
COPY --link ./.env ./.env

CMD ["rust-webapp"]