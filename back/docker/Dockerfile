FROM rust:buster as build

WORKDIR /tmp/build

RUN apt-get update && apt-get install -y libsqlite3-dev

COPY . .

RUN cargo build --release

####### 

FROM rust:buster

RUN apt-get update && apt-get install -y libsqlite3-dev

WORKDIR /usr/src/app

COPY --from=build /tmp/build/target/release/back ./app

RUN cargo install diesel_cli --no-default-features --features sqlite

EXPOSE 8080