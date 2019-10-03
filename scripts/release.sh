#!/usr/bin/env bash

function checkout_master() {
  git checkout master
  if [[ "$?" != "0" ]]; then
    printf "%s.\n" "Failed to checkout the master branch, not continuing"
    exit 1
  fi
}

function push() {
  checkout_master
  git push --follow-tags
  exit 0
}

function tag() {
  local tag_name="$1"; shift
  checkout_master
  git tag -s ${1}
  sed -i.bk "s/version = \"[0-9].[0-9].[0-9]\"/version = \""${tag_name}"\"/" Cargo.toml
  rm Cargo.toml.bk
  sed -i.bk "s/LABEL com.cezarmathe.feeder-version=\"[0-9].[0-9].[0-9]\"/LABEL com.cezarmathe.feeder-version=\""${tag_name}"\"/" docker/Dockerfile
  rm docker/Dockerfile.bk
  exit 0
}

function merge() {
  checkout_master
  git merge develop
  exit 0
}

function main() {
  local action="$1"; shift
  local action_param="$1"; shift

  if [[ -z "${action}" ]]; then
    printf "%s.\n" "Missing action. Try merge, tag or push"
    exit 1
  fi

  if [[ "${action}" == "merge" ]]; then
    merge
    exit 0
  fi

  if [[ "${action}" == "tag" ]]; then
    if [[ -z "${action_param}" ]]; then
      printf "%s.\n" "The tag command requires a another parameter: the tag that is used to merge the latest commit"
      exit 1
    fi
    tag ${action_param}
    exit 0
  fi

  if [[ "${action}" == "push" ]]; then
    push
    exit 0
  fi

  printf "%s\n" "Unknown command. Try merge, tag or push"
  exit 1
}

main $@
