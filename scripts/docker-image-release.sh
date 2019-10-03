#!/usr/bin/env bash

function develop() {
  docker build -t feeder:develop -f docker/Dockerfile-dev .
}

function release() {
  local tag="$1"; shift
  docker build -t feeder:${tag} -f docker/Dockerfile .
}

function main() {
  local release_type="$1"; shift
  local arg="$1"; shift

  if [[ -z "${release_type}" ]]; then
    printf "%s.\n" "Missing release type. Try develop or release"
    exit 1
  fi

  if [[ "${release_type}" == "release" ]]; then
    if [[ -z "${arg}" ]]; then
      printf "%s.\n" "Missing tag argument for the release docker image"
      exit 1
    fi
    release ${arg}
    exit 0
  fi

  if [[ "${release_type}" == "develop" ]]; then
    develop
    exit 0
  fi

  printf "%s\n" "Unknown release type. Try develop or release"
  exit 1
}

main $@