FROM rust:latest as build
COPY ./ ./
# BUILD COMMAND
RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/rust-w6d /build-out/

FROM ubuntu:18.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=build /build-out/rust-w6d /
# START COMMAND
CMD ["./rust-w6d"]