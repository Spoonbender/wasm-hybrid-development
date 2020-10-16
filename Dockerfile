FROM rust:1.29.0

RUN cargo install wasm-pack && cargo install cargo-generate