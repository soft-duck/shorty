FROM docker.io/archlinux as builder
RUN pacman -Syu --noconfirm base-devel rustup
RUN rustup default stable && rustup target add wasm32-unknown-unknown
RUN cargo install cargo-make trunk
WORKDIR /build
COPY . ./
RUN cargo make -p release


FROM docker.io/archlinux
WORKDIR /root
COPY --from=builder /build/target/release/shorty .
CMD ["./shorty"]
