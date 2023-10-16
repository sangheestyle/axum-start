FROM clux/muslrust as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/x86_64-unknown-linux-musl/release/axum-start /
CMD ["./axum-start"]
