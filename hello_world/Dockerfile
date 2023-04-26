FROM rust:1.67.1-alpine3.17 AS build

RUN cargo new app
WORKDIR app

COPY Cargo.* .
RUN cargo build --release

RUN rm -r src
COPY . .
RUN cargo build --release

FROM scratch

WORKDIR /app
COPY --from=build app/target/release/hello_world .

CMD ["./hello_world"]
