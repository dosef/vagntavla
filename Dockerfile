From rust:1.46 as builder
WORKDIR /vagntavla
copy . .
RUN rustup update nightly;
RUN rustup default nightly;
RUN cargo build --release

From centos:8
WORKDIR /vagntavla
copy --from=builder vagntavla/target/release/vagntavla .
copy --from=builder vagntavla/static ./static
copy --from=builder vagntavla/templates ./templates
CMD ["./vagntavla"]
