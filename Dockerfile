FROM ubuntu:22.04

# Install required packages
RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    qemu-system-x86 \
    xorriso \
    mtools \
    grub-pc-bin \
    grub-common \
    llvm \
    clang \
    lld \
    pkg-config \
    python3-pip \
    gcc-multilib \
    && rm -rf /var/lib/apt/lists/*

# Install rustup and pin nightly
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain none
ENV PATH="/root/.cargo/bin:${PATH}"

# Pin to the specific nightly and install needed components
RUN rustup toolchain install nightly-2024-09-01 && \
    rustup default nightly-2024-09-01 && \
    rustup component add rust-src llvm-tools-preview --toolchain nightly-2024-09-01

WORKDIR /src