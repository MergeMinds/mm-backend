FROM rust:1.81-slim AS builder

# Prepare toolchain (build for x86_64)
RUN apt update && apt-get install -y gcc-x86-64-linux-gnu
RUN rustup target add x86_64-unknown-linux-musl
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc

WORKDIR /mergeminds

# Fetch & cache dependencies
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src/ && \
    echo 'fn main() { println!("You should not see this") }' > src/main.rs \
    cargo fetch --target x86_64-unknown-linux-musl

ADD src src
COPY .sqlx .sqlx

# Build the binary
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM --platform=linux/amd64 scratch AS final

WORKDIR /mergeminds
COPY --from=builder /mergeminds/target/x86_64-unknown-linux-musl/release/mm-backend .

ENTRYPOINT ["./mm-backend"]
