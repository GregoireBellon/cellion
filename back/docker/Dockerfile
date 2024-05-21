FROM rust:alpine as build

RUN apk add sqlite-static sqlite-dev musl-dev

WORKDIR /usr/src/build

COPY . .
RUN cargo build --release

####### 

FROM alpine 

RUN apk add --no-cache sqlite musl

WORKDIR /usr/src/app

COPY --from=build /usr/src/build/target/release/back ./app

EXPOSE 8080

ENTRYPOINT [ "./app" ]