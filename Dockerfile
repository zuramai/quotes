FROM rust:1.70-bullseye


RUN cargo new --bin quotes
WORKDIR /quotes

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml 
COPY ./.env ./.env
COPY ./.sqlx ./.sqlx

RUN cargo build --release
RUN rm src/*.rs 

COPY ./src ./src 

RUN rm ./target/release/quotes 
RUN cargo install --path .

CMD ["./target/release/quotes"]