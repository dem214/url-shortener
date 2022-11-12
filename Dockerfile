FROM rust:latest as buider

WORKDIR /usr/src/url-shortener

# install dependencies
RUN cargo init 
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo fetch --locked

# build binary
COPY src src
RUN cargo build --release --offline

FROM debian:buster
COPY --from=builder /usr/src/url-shortener/target/release/url-shortener /usr/local/bin/url-shortener
CMD ["url-shortener"]
