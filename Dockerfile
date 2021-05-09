FROM rustlang/rust:nightly-buster as builder

WORKDIR app
COPY . .

RUN cargo build --release

FROM bitnami/minideb:buster as runtime

RUN install_packages libssl1.1 ca-certificates
COPY --from=builder /app/target/release/zephir /usr/local/bin/

USER 1000
ENTRYPOINT ["/usr/local/bin/zephir"]
