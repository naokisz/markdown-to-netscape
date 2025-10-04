FROM rust:latest

# Install common build tools and certificates
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
       build-essential \
       ca-certificates \
       pkg-config \
       curl \
       git \
    && rm -rf /var/lib/apt/lists/*

# Ensure rustup components are available (clippy, rustfmt)
RUN rustup component add clippy rustfmt || true

ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup

WORKDIR /workspace

CMD ["/bin/ash"]
