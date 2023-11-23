# fetch the vendor with the builder platform to avoid qemu issues
FROM --platform=$BUILDPLATFORM rust:1-bookworm AS vendor

ENV USER=root

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock

# https://docs.docker.com/engine/reference/builder/#run---mounttypecache
RUN --mount=type=cache,target=$CARGO_HOME/git,sharing=locked \
  --mount=type=cache,target=$CARGO_HOME/registry,sharing=locked \
  mkdir -p /code/.cargo \
  && cargo vendor > /code/.cargo/config

FROM rust:1-bookworm AS base

RUN rustup toolchain install nightly \
    && rustup default nightly

ENV USER=root

WORKDIR /code

COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock
COPY src /code/src
COPY --from=vendor /code/.cargo /code/.cargo
COPY --from=vendor /code/vendor /code/vendor

CMD [ "cargo", "test", "--offline" ]

FROM base AS builder

RUN cargo build --release --offline

FROM scratch AS binary

COPY --from=builder /code/target/release/mini-agent /mini-agent

FROM debian:stable-slim

LABEL maintaner="Jeremie Drouet <jeremie.drouet@gmail.com>"

ENV RUST_LOG=info

COPY --from=builder /code/target/release/mini-agent /usr/bin/mini-agent

EXPOSE 3000

ENTRYPOINT [ "/usr/bin/mini-agent" ]
CMD [ "--config", "/etc/mini-agent/config.toml" ]