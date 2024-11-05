#!/bin/sh

set -e -x

echo "Installing dependencies"

apt-get update

DEBIAN_FRONTEND="noninteractive" apt-get -y install tzdata

apt-get install -y software-properties-common wget curl

wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
add-apt-repository "deb http://apt.llvm.org/noble/ llvm-toolchain-noble-18 main"

apt-get update

apt-get install -y \
    build-essential \
    gcc-riscv64-unknown-elf \
    binutils-riscv64-unknown-elf \
    gdb-multiarch \
    qemu-system-misc \
    vim \
    tmux \
    git \
    clangd-18 \
    bear

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path

# Add Rust to PATH
export PATH="/root/.cargo/bin:${PATH}"
export CARGO_HOME="/root/.cargo"

# Source the cargo environment
. "/root/.cargo/env"

# Add RISC-V target
rustup target add riscv64gc-unknown-none-elf

apt-get clean

# Make sure Rust binaries are available system-wide
echo 'export PATH="/root/.cargo/bin:${PATH}"' >> /etc/profile
echo 'export CARGO_HOME="/root/.cargo"' >> /etc/profile
