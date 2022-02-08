# FROM rust:1.56.0 AS cacher
FROM ekidd/rust-musl-builder:stable AS builder

WORKDIR /user/test-iced
COPY Cargo.toml Cargo.toml

# for caching
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo 'fn main(){println!("cached!")}' > src/main.rs

RUN cargo build --release --target x86_64-unknown-linux-musl

RUN rm src/main.rs

# copy code
COPY src/ src/

# for recompile
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/test_iced*

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/libtest_iced*

RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip /user/test-iced/target/x86_64-unknown-linux-musl/release/test-iced

# FROM ekidd/rust-musl-builder:stable
FROM rust:1.57-slim

COPY --from=builder \
    /user/test-iced/target/x86_64-unknown-linux-musl/release/test-iced /
RUN mkdir target

ENTRYPOINT [ "/test-iced" ]