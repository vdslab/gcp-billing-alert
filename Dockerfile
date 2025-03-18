# Use the official Rust image as a builder
FROM rust:slim as builder

# Create a new empty shell project
WORKDIR /app
COPY . .

# Build the application with release profile
RUN cargo build --release

# Use a slim image for the runtime
FROM debian:bullseye-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/gcp-billing-alert /usr/local/bin/gcp-billing-alert

# Run the binary
CMD ["gcp-billing-alert"]
