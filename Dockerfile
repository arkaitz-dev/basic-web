# Build stage
FROM rust:1-slim as builder

# Install musl target for static compilation
RUN rustup target add x86_64-unknown-linux-musl

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build static binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM scratch

# Copy static binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/basic-web /basic-web

# Copy static files
COPY static /static

# Expose port
EXPOSE 3000

# Run the application
CMD ["/basic-web"]