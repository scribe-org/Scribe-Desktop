set shell := ["bash", "-cu"]

default:
    just pre_commit

pre_commit: fmt-check clippy build test

fmt:
    cd scribe && cargo fmt --all

fmt-check:
    cd scribe && cargo fmt --all -- --check

clippy:
    cd scribe && cargo clippy --all-targets --all-features -- -D warnings

build:
    cd scribe && cargo build --verbose

test:
    cd scribe && cargo test --verbose

install-deps:
    sudo apt-get update
    sudo apt-get install -y \
        pkg-config \
        libxi-dev \
        libx11-dev \
        libxrandr-dev \
        libxinerama-dev \
        libxcursor-dev \
        libgl1-mesa-dev \
        libxtst-dev

install-rust-tools:
    rustup component add rustfmt clippy

