FROM rust:1.73-slim-buster as builder
ENV AR_x86_64_unknown_linux_musl=llvm-ar
RUN apt update && apt install -y musl-tools musl-dev build-essential gcc-x86-64-linux-gnu clang llvm
RUN update-ca-certificates
RUN rustup target add x86_64-unknown-linux-musl
COPY . /app
RUN \
  --mount=type=cache,target=/app/target,rw \
  --mount=type=cache,target=/usr/local/cargo/registry,rw
RUN cd /app && \
    cargo build --target x86_64-unknown-linux-musl --release
RUN cp /app/target/x86_64-unknown-linux-musl/release/axum-start /app/axum-start

FROM alpine:3.18.4
RUN mkdir /app
COPY --from=builder /app/axum-start /app/axum-start
WORKDIR /app
WORKDIR /app
CMD ["./axum-start"]
