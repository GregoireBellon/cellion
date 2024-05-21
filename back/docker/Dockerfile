FROM rust:buster

RUN apt-get update && apt-get install -y sqlite3 libsqlite3-dev
RUN cargo install diesel_cli --no-default-features --features sqlite

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

RUN diesel setup
RUN diesel migration run

EXPOSE 8080