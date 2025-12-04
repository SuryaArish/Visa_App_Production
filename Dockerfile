FROM rust:slim

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy source
COPY . .

# Build with optimizations for faster compile
RUN cargo build --release

# Expose port
EXPOSE 3000

# Run the binary
CMD ["./target/release/visa-api"]