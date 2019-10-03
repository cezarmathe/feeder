#!/usr/bin/env bash

function run_build() {
  cargo build --release
  if [[ "$?" != 0 ]]; then
    printf "%s.\n" "Failed to build the binary."
    exit 1
  fi
}

function login_github() {
  docker login docker.pkg.github.com -u ${GITHUB_USERNAME} -p ${GITHUB_TOKEN}
  if [[ "$?" != 0 ]]; then
    printf "%s.\n" "Failed to log in the github package registry."
    exit 1
  fi
}

function develop() {
  run_build

  docker build -t feeder:develop -f docker/Dockerfile-dev .
  if [[ "$?" != 0 ]]; then
    printf "%s.\n" "Failed to build the docker image"
    exit 1
  fi

  login_github

  docker tag feeder:latest docker.pkg.github.com/${GITHUB_USERNAME}/feeder/feeder:develop
  if [[ "$?" != 0 ]]; then
    printf "%s.\n" "Failed to tag the docker image"
    exit 1
  fi

  docker push docker.pkg.github.com/${GITHUB_USERNAME}/feeder/feeder:develop
  if [[ "$?" != 0 ]]; then
    printf "%s.\n" "Failed to push the docker image"
    exit 1
  fi
}

function release() {
  local tag="$1"; shift

#  build image

  login_github

#  push image
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
