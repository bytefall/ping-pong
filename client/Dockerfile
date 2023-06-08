# build
FROM rust:1.70-bullseye as build

WORKDIR /app/build

COPY ./ ./

RUN apt-get update
RUN apt-get install -y cmake libclang1-11 libclang-common-11-dev
RUN cargo build --release --bin client
RUN mkdir /app/rel
RUN cp ./target/release/client /app/rel
RUN cp ./*.pem /app/rel

# release
FROM ubuntu as release

WORKDIR /app
COPY --from=build /app/rel .

ENTRYPOINT ["./client"]
