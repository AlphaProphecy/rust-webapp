FROM rust:1.73

WORKDIR /usr/src/rust-webapp
COPY . .

RUN cargo install --path .

CMD ["rust-webapp"]