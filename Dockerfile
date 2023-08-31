FROM rust:latest

WORKDIR /usr/src/rustacean

RUN cargo install diesel_cli --no-default-features --features postgres 
RUN rustup update
RUN rustup default nightly

COPY Cargo.toml Cargo.lock ./

COPY . .

CMD ["cargo", "run", "--release"]
