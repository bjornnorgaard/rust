FROM rust:1.67.1-alpine3.17

WORKDIR app

COPY . .

# Optimized build is NOT the default
RUN cargo build --release

# App name set in Cargo.toml => package.name
CMD ["./target/release/hello_world"]
