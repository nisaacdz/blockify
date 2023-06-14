FROM rust:latest

# Install Rust nightly
RUN rustup toolchain install nightly

# Set Rust nightly as the default toolchain
RUN rustup default nightly

# Install Rust stable
RUN rustup toolchain install stable