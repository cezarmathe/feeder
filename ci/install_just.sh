#!/usr/bin/env bash

curl -LSfs https://japaric.github.io/trust/install.sh | \
    sh -s -- --git casey/just --target x86_64-unknown-linux-musl --to .

sudo mv just /usr/bin/just
