#!/usr/bin/env bash
ver=$1
echo "version $ver"
alias rust-musl-builder='docker run --rm -it  -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder:stable'
rust-musl-builder cargo build --release
docker build -t data-gen:${ver} .