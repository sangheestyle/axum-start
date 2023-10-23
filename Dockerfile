# build container
FROM rust:1.68.2-slim-buster as builder
RUN apt update && apt install -y librust-openssl-dev libssl-dev 
WORKDIR /app
COPY . /app
RUN cargo build --release

# target container
FROM rust:1.68.2-slim-buster
RUN mkdir /app
COPY --from=builder /app/target/release/axum-start /app/axum-start
WORKDIR /app
CMD ["./axum-start"]
EXPOSE 8000