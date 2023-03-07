FROM docker.io/clux/muslrust:stable as builder
WORKDIR /build
COPY . ./
RUN cargo build --profile production


FROM scratch
WORKDIR /root
COPY --from=builder /build/target/x86_64-unknown-linux-musl/production/shorty .
CMD ["./shorty"]