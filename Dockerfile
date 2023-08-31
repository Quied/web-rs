FROM rust:1.54

WORKDIR /rustacean

RUN cargo install diesel_cli --no-default-features --features postgres 

COPY Cargo.toml Cargo.lock ./

COPY . .

CMD ["cargo", "run", "--release"]