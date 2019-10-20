#!/usr/bin/env bash

docker run \
    -v "$PWD":/build \
    -v "$HOME"/.cargo/git:/root/.cargo/git \
    -v "$HOME"/.cargo/registry:/root/.cargo/registry \
    fredrikfornwall/rust-static-builder-nightly

if [[ ! -f "target/x86_64-unknown-linux-musl/release/feeder" ]]
then
    echo "Binary not built"
    exit 1
fi

