#!/usr/bin/env bash

cargo install just

if [[ "$?" == "0" ]]; then
    exit 0
fi

if [[ "$?" == "101" ]]; then
    printf "%s\n" "Just is already installed."
    exit 0
fi

printf "%s\n" "Failed to install Just."

exit $?
