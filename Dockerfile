# Builder stage
FROM rustlang/rust:nightly-bookworm AS builder

# Install wasm32 target
RUN rustup target add wasm32-unknown-unknown

# Install cargo-leptos
RUN cargo install --locked cargo-leptos

# Create app directory
WORKDIR /app

# Copy the whole project
COPY . .

# Build the application
# We use --release for the final production build
RUN cargo leptos build --release -vv

# Runtime stage
FROM debian:bookworm-slim AS runtime

# Install CA certificates and other runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the server binary from the builder stage
# The binary name depends on the package name in Cargo.toml
COPY --from=builder /app/target/server/release/sharp-system /app/sharp-system

# Copy the site assets (JS, WASM, CSS)
COPY --from=builder /app/target/site /app/site

# Set environment variables for Leptos
ENV LEPTOS_SITE_ROOT=site
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV LEPTOS_ENV=PROD

# Expose the port
EXPOSE 3000

# Run the server
CMD ["/app/sharp-system"]
