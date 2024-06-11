FROM rust:latest as builder
WORKDIR /api
COPY . /api/
RUN cargo build --release

FROM rust:latest as final
COPY --from=builder /api/target/release/api /usr/local/bin/api
ENTRYPOINT ["api"]