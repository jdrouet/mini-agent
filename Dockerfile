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
RUN cargo init --lib --name mini-agent-sink-prelude mini-agent-sink-prelude
COPY mini-agent-sink-prelude/Cargo.toml /code/mini-agent-sink-prelude/Cargo.toml
RUN cargo init --lib --name mini-agent-source-prelude mini-agent-source-prelude
COPY mini-agent-source-prelude/Cargo.toml /code/mini-agent-source-prelude/Cargo.toml
RUN cargo init --lib --name mini-agent-source-sysinfo mini-agent-source-sysinfo
COPY mini-agent-source-sysinfo/Cargo.toml /code/mini-agent-source-sysinfo/Cargo.toml
RUN cargo init --lib --name mini-agent-source-timer mini-agent-source-timer
COPY mini-agent-source-timer/Cargo.toml /code/mini-agent-source-timer/Cargo.toml

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
COPY mini-agent-sink-prelude/Cargo.toml /code/mini-agent-sink-prelude/Cargo.toml
COPY mini-agent-sink-prelude/src /code/mini-agent-sink-prelude/src
COPY mini-agent-source-prelude/Cargo.toml /code/mini-agent-source-prelude/Cargo.toml
COPY mini-agent-source-prelude/src /code/mini-agent-source-prelude/src
COPY mini-agent-source-sysinfo/Cargo.toml /code/mini-agent-source-sysinfo/Cargo.toml
COPY mini-agent-source-sysinfo/src /code/mini-agent-source-sysinfo/src
COPY mini-agent-source-timer/Cargo.toml /code/mini-agent-source-timer/Cargo.toml
COPY mini-agent-source-timer/src /code/mini-agent-source-timer/src
COPY --from=vendor /code/.cargo /code/.cargo
COPY --from=vendor /code/vendor /code/vendor

CMD [ "cargo", "test", "--offline" ]

FROM base AS builder

RUN cargo build --release --offline

FROM scratch AS binary

COPY --from=builder /code/target/release/mini-agent /mini-agent

FROM rust:1-bookworm AS deb-build

RUN rustup toolchain install nightly \
    && rustup default nightly \
    && cargo install cargo-deb

ENV USER=root

WORKDIR /code

COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock

COPY mini-agent/Cargo.toml /code/mini-agent/Cargo.toml
COPY mini-agent/assets /code/mini-agent/assets
COPY mini-agent/src /code/mini-agent/src
COPY mini-agent/LICENSE /code/mini-agent/LICENSE
COPY mini-agent/readme.md /code/mini-agent/readme.md

COPY mini-agent-core/Cargo.toml /code/mini-agent-core/Cargo.toml
COPY mini-agent-core/src /code/mini-agent-core/src

COPY mini-agent-sink-prelude/Cargo.toml /code/mini-agent-sink-prelude/Cargo.toml
COPY mini-agent-sink-prelude/src /code/mini-agent-sink-prelude/src

COPY mini-agent-source-prelude/Cargo.toml /code/mini-agent-source-prelude/Cargo.toml
COPY mini-agent-source-prelude/src /code/mini-agent-source-prelude/src

COPY mini-agent-source-sysinfo/Cargo.toml /code/mini-agent-source-sysinfo/Cargo.toml
COPY mini-agent-source-sysinfo/src /code/mini-agent-source-sysinfo/src

COPY mini-agent-source-timer/Cargo.toml /code/mini-agent-source-timer/Cargo.toml
COPY mini-agent-source-timer/src /code/mini-agent-source-timer/src

COPY --from=vendor /code/.cargo /code/.cargo
COPY --from=vendor /code/vendor /code/vendor

RUN cargo deb -p mini-agent

FROM scratch as deb

COPY --from=deb-build /code/target/debian /target/debian

FROM debian:stable-slim

LABEL maintaner="Jeremie Drouet <jeremie.drouet@gmail.com>"

ENV RUST_LOG=info

COPY --from=builder /code/target/release/mini-agent /usr/bin/mini-agent
COPY mini-agent/assets/config.toml /etc/mini-agent/config.toml

EXPOSE 3000

ENTRYPOINT [ "/usr/bin/mini-agent" ]