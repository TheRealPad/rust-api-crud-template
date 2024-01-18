FROM rust:1.69-buster as builder
WORKDIR /app
ARG API_PORT
ENV API_PORT=$API_PORT
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY --from=builder /app/target/release/api-template .
CMD ["./api-template"]