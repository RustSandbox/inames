FROM rust:1.82-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && \
    cargo fetch && \
    cargo build --release && \
    rm -rf src
COPY . .
RUN cargo build --release

CMD ["./target/release/inames"] 