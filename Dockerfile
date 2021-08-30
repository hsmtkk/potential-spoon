FROM hsmtkk/cargo-chef:1.54 AS chef
WORKDIR /opt

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /opt/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
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
