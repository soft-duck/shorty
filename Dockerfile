FROM docker.io/clux/muslrust:stable as builder
RUN rustup default stable && rustup target add wasm32-unknown-unknown
RUN cargo install cargo-make trunk
WORKDIR /build
COPY . ./
RUN cargo make -p release


FROM scratch
WORKDIR /root
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/shorty .
CMD ["./shorty"]
