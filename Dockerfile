FROM rust:1.54 AS builder

RUN cargo build --release

FROM gcr.io/distroless/static-debian10 AS runtime
