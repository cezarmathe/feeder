#!/usr/bin/env just --justfile

test: build check
	cargo test

build:
	cargo build

check:
	cargo check

test-release: build-release check
	cargo test --release --verbose

build-release:
	cargo build --release --verbose

install-dev-deps:
	# rustup install nightly
	rustup override set nightly
	# rustup update nightly
	cargo install clippy
	cargo install rustfmt

lint: build
	cargo clippy
	echo Checking for FIXME/TODO...
	! grep --color -En 'FIXME|TODO' src/*.rs
	echo Checking for long lines...
	! grep --color -En '.{101}' src/*.rs

format:
	cargo fmt

# upload the binary artifact to the current release on github
binary-artifact TAG: test
	@echo "Use https://github.com/aktau/github-release for automatically uploading artifacts."
	exit 1

# login into the github docker package registry
_docker_login:
	@docker login docker.pkg.github.com -u ${GITHUB_USERNAME} -p ${GITHUB_TOKEN}

# build the develop docker image
docker-image-develop: test _docker_login
	docker build -t feeder:develop -f docker/Dockerfile-dev .
	docker tag feeder:develop docker.pkg.github.com/${GITHUB_USERNAME}/feeder/feeder:develop
	docker push docker.pkg.github.com/${GITHUB_USERNAME}/feeder/feeder:develop

# build the release docker image, requires the tag
docker_image_release TAG: test _docker_login
	docker build -t feeder:{{TAG}} -f docker/Dockerfile .
	docker tag feeder:{{TAG}} docker.pkg.github.com/${GITHUB_USERNAME}/feeder/feeder:{{TAG}}
	docker push docker.pkg.github.com/${GITHUB_USERNAME}/feeder/feeder:{{TAG}}

