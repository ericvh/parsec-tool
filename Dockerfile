FROM rust:buster as builder

WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
VOLUME /run/parsec
COPY --from=builder /usr/src/myapp/target/release/parsec-tool /usr/local/bin/parsec-tool
ENTRYPOINT ["/usr/local/bin/parsec-tool"]