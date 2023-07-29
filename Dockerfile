FROM rust as builder

COPY . /app

WORKDIR /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /app/target/release/auth /app/auth
COPY --from=builder /app/Rocket.toml /app/Rocket.toml

WORKDIR /app

CMD ["./auth"]