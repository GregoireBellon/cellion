FROM rust:alpine as build

RUN apk add --no-cache sqlite-static sqlite-dev musl-dev

WORKDIR /usr/src/build

COPY . .
RUN cargo build --release

####### 

FROM alpine 

RUN apk add --no-cache sqlite musl

WORKDIR /usr/src/app

RUN mkdir data

COPY --from=build /usr/src/build/target/release/back ./app

EXPOSE 8080

ENTRYPOINT [ "./app" ]