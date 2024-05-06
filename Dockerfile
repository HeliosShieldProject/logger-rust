FROM clux/muslrust:stable AS chef
ARG LOGGER_PORT
RUN cargo install cargo-chef
WORKDIR /logger

FROM chef AS planner
COPY . .
CMD [ "mv", ".env.production", ".env" ]
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /logger/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
CMD [ "mv", ".env.production", ".env" ]
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
LABEL org.opencontainers.image.source https://github.com/HeliosShieldProject/logger-rust
COPY --from=builder /logger/target/x86_64-unknown-linux-musl/release/logger /logger
COPY --from=builder /logger/.env.production .env
ENTRYPOINT ["/logger"]
EXPOSE $LOGGER_PORT