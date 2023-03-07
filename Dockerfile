FROM docker.io/rust:1.67-alpine as builder
RUN apk add --no-cache musl-dev
WORKDIR /build
COPY . ./
RUN cargo build


FROM docker.io/alpine
WORKDIR /root
COPY --from=builder /build/target/debug/shorty .
CMD ["./shorty"]