#!/usr/bin/env bash

sed -i.bk "s/^version = \"[0-9].[0-9].[0-9]\"/version = \"${1}\"/" Cargo.toml
rm Cargo.toml.bk
