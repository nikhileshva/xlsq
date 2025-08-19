# Multi-stage build for smaller image
FROM rust:1.75 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

# Install required dependencies for Excel files
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/xlsq /usr/local/bin/xlsq

# Create a non-root user
RUN useradd -r -s /bin/false xlsq

USER xlsq
WORKDIR /data

ENTRYPOINT ["xlsq"]
CMD ["--help"]