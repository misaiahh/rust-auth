# STAGE 1 - generate a recipe file for dependencies
FROM rust as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# STAGE 2 - build dependencies
FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# STAGE 3 - build source with cached dependencies
FROM rust as builder

ENV USER=web
ENV UID=1001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

COPY . /app
WORKDIR /app
COPY --from=cacher /app/target target
RUN cargo build --release

# STAGE 4 - build docker image
FROM gcr.io/distroless/cc-debian11

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

COPY --from=builder /app/target/release/auth /app/auth
COPY --from=builder /app/Rocket.toml /app/Rocket.toml

WORKDIR /app

USER web:web

CMD ["./auth"]