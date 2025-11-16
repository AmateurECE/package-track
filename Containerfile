FROM docker.io/library/rust:alpine3.22
WORKDIR /src
COPY . /src
RUN apk add build-base zlib-static zlib-dev openssl-libs-static openssl-dev
RUN cargo build --release

FROM alpine:3.22
COPY --from=0 /src/target/release/package-track /bin/
ENTRYPOINT ["/bin/package-track"]
