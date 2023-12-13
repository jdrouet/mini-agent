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
# source
RUN cargo init --lib --name mini-agent-source-prelude mini-agent-source-prelude
COPY mini-agent-source-prelude/Cargo.toml /code/mini-agent-source-prelude/Cargo.toml
RUN cargo init --lib --name mini-agent-source-http-server mini-agent-source-http-server
COPY mini-agent-source-http-server/Cargo.toml /code/mini-agent-source-http-server/Cargo.toml
RUN cargo init --lib --name mini-agent-source-random-logs mini-agent-source-random-logs
COPY mini-agent-source-random-logs/Cargo.toml /code/mini-agent-source-random-logs/Cargo.toml
RUN cargo init --lib --name mini-agent-source-random-metrics mini-agent-source-random-metrics
COPY mini-agent-source-random-metrics/Cargo.toml /code/mini-agent-source-random-metrics/Cargo.toml
RUN cargo init --lib --name mini-agent-source-sysinfo mini-agent-source-sysinfo
COPY mini-agent-source-sysinfo/Cargo.toml /code/mini-agent-source-sysinfo/Cargo.toml
# transform
RUN cargo init --lib --name mini-agent-transform-prelude mini-agent-transform-prelude
COPY mini-agent-transform-prelude/Cargo.toml /code/mini-agent-transform-prelude/Cargo.toml
RUN cargo init --lib --name mini-agent-transform-filter mini-agent-transform-filter
COPY mini-agent-transform-filter/Cargo.toml /code/mini-agent-transform-filter/Cargo.toml
# sink
RUN cargo init --lib --name mini-agent-sink-prelude mini-agent-sink-prelude
COPY mini-agent-sink-prelude/Cargo.toml /code/mini-agent-sink-prelude/Cargo.toml
RUN cargo init --lib --name mini-agent-sink-console mini-agent-sink-console
COPY mini-agent-sink-console/Cargo.toml /code/mini-agent-sink-console/Cargo.toml
RUN cargo init --lib --name mini-agent-sink-datadog mini-agent-sink-datadog
COPY mini-agent-sink-datadog/Cargo.toml /code/mini-agent-sink-datadog/Cargo.toml

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
# source
COPY mini-agent-source-prelude/Cargo.toml /code/mini-agent-source-prelude/Cargo.toml
COPY mini-agent-source-prelude/src /code/mini-agent-source-prelude/src
COPY mini-agent-source-http-server/Cargo.toml /code/mini-agent-source-http-server/Cargo.toml
COPY mini-agent-source-http-server/src /code/mini-agent-source-http-server/src
COPY mini-agent-source-random-logs/Cargo.toml /code/mini-agent-source-random-logs/Cargo.toml
COPY mini-agent-source-random-logs/src /code/mini-agent-source-random-logs/src
COPY mini-agent-source-random-metrics/Cargo.toml /code/mini-agent-source-random-metrics/Cargo.toml
COPY mini-agent-source-random-metrics/src /code/mini-agent-source-random-metrics/src
COPY mini-agent-source-sysinfo/Cargo.toml /code/mini-agent-source-sysinfo/Cargo.toml
COPY mini-agent-source-sysinfo/src /code/mini-agent-source-sysinfo/src
# transform
COPY mini-agent-transform-prelude/Cargo.toml /code/mini-agent-transform-prelude/Cargo.toml
COPY mini-agent-transform-prelude/src /code/mini-agent-transform-prelude/src
COPY mini-agent-transform-filter/Cargo.toml /code/mini-agent-transform-filter/Cargo.toml
COPY mini-agent-transform-filter/src /code/mini-agent-transform-filter/src
# sink
COPY mini-agent-sink-prelude/Cargo.toml /code/mini-agent-sink-prelude/Cargo.toml
COPY mini-agent-sink-prelude/src /code/mini-agent-sink-prelude/src
COPY mini-agent-sink-console/Cargo.toml /code/mini-agent-sink-console/Cargo.toml
COPY mini-agent-sink-console/src /code/mini-agent-sink-console/src
COPY mini-agent-sink-datadog/Cargo.toml /code/mini-agent-sink-datadog/Cargo.toml
COPY mini-agent-sink-datadog/src /code/mini-agent-sink-datadog/src
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
# source
COPY mini-agent-source-prelude/Cargo.toml /code/mini-agent-source-prelude/Cargo.toml
COPY mini-agent-source-prelude/src /code/mini-agent-source-prelude/src
COPY mini-agent-source-http-server/Cargo.toml /code/mini-agent-source-http-server/Cargo.toml
COPY mini-agent-source-http-server/src /code/mini-agent-source-http-server/src
COPY mini-agent-source-random-logs/Cargo.toml /code/mini-agent-source-random-logs/Cargo.toml
COPY mini-agent-source-random-logs/src /code/mini-agent-source-random-logs/src
COPY mini-agent-source-random-metrics/Cargo.toml /code/mini-agent-source-random-metrics/Cargo.toml
COPY mini-agent-source-random-metrics/src /code/mini-agent-source-random-metrics/src
COPY mini-agent-source-sysinfo/Cargo.toml /code/mini-agent-source-sysinfo/Cargo.toml
COPY mini-agent-source-sysinfo/src /code/mini-agent-source-sysinfo/src
# transforms
COPY mini-agent-transform-prelude/Cargo.toml /code/mini-agent-transform-prelude/Cargo.toml
COPY mini-agent-transform-prelude/src /code/mini-agent-transform-prelude/src
COPY mini-agent-transform-filter/Cargo.toml /code/mini-agent-transform-filter/Cargo.toml
COPY mini-agent-transform-filter/src /code/mini-agent-transform-filter/src
# sink
COPY mini-agent-sink-prelude/Cargo.toml /code/mini-agent-sink-prelude/Cargo.toml
COPY mini-agent-sink-prelude/src /code/mini-agent-sink-prelude/src
COPY mini-agent-sink-console/Cargo.toml /code/mini-agent-sink-console/Cargo.toml
COPY mini-agent-sink-console/src /code/mini-agent-sink-console/src
COPY mini-agent-sink-datadog/Cargo.toml /code/mini-agent-sink-datadog/Cargo.toml
COPY mini-agent-sink-datadog/src /code/mini-agent-sink-datadog/src

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