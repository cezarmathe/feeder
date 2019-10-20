#!/usr/bin/env bash

curl -L  \
    -o github-release.tar.bz2 \
    https://github.com/aktau/github-release/releases/download/v0.7.1/linux-amd64-github-release.tar.bz2

tar -xjf github-release.tar.bz2

sudo mv bin/linux/amd64/github-release /usr/bin/github-release
