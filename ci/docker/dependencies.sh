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
    \
    vim \
    tmux \
    git \
    \
    clangd-18 \
    bear

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

. $HOME/.cargo/env

rustup target add riscv64gc-unknown-none-elf

apt-get clean
