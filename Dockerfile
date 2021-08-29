FROM rust:1.54 AS builder

WORKDIR /opt

COPY src /opt/src
COPY Cargo.lock /opt/Cargo.lock
COPY Cargo.toml /opt/Cargo.toml

RUN cargo build --release

FROM gcr.io/distroless/cc-debian10 AS runtime

WORKDIR /opt

COPY --from=builder /opt/target/release/potential-spoon /opt/potential-spoon
COPY template /opt/template
COPY static /opt/static

ENV LISTEN_ADDRESS=0.0.0.0 \
 LISTEN_PORT=80 \
 RUST_LOG=info

EXPOSE 80

ENTRYPOINT ["/opt/potential-spoon"]
