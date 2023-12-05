# fetch the vendor with the builder platform to avoid qemu issues
FROM --platform=$BUILDPLATFORM rust:1-bookworm AS vendor

ENV USER=root

WORKDIR /code
COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock
RUN cargo init --bin --name mini-agent mini-agent
COPY mini-agent/Cargo.toml /code/mini-agent/Cargo.toml
RUN cargo init --lib --name mini-agent-core mini-agent-core
COPY mini-agent-core/Cargo.toml /code/mini-agent-core/Cargo.toml

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
COPY mini-agent/Cargo.toml /code/mini-agent/Cargo.toml
COPY mini-agent/src /code/mini-agent/src
COPY mini-agent-core/Cargo.toml /code/mini-agent-core/Cargo.toml
COPY mini-agent-core/src /code/mini-agent-core/src
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