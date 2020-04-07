FROM docker.io/ekidd/rust-musl-builder:1.42.0 as builder

WORKDIR /home/rust/src
# Caching rust dependencies start
COPY Cargo.toml Cargo.lock ./
RUN mkdir src/ \
    && echo "fn main() { }" > src/main.rs \
    && cargo build --release \
    && rm ./target/x86_64-unknown-linux-musl/release/deps/rs_minecraft_exporter*
# Caching rust dependencies end

COPY src ./src

RUN cargo build --release
RUN strip /home/rust/src/target/x86_64-unknown-linux-musl/release/rs-minecraft-exporter

FROM docker.io/alpine:latest
STOPSIGNAL SIGINT
WORKDIR /usr/src/app

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/rs-minecraft-exporter /usr/src/app/rs-minecraft-exporter

EXPOSE 8000

ENTRYPOINT ["/usr/src/app/rs-minecraft-exporter", "/world"]
