#!/usr/bin/env just --justfile

test: build check
	cargo test

build:
	cargo build

check:
	cargo check

run: test
	cargo run

test-release: build-release check-release
	cargo test --release --verbose

build-release:
	cargo build --release --verbose

check-release:
	cargo check --release --verbose

run-release: test-release
	cargo run --release

clean:
	cargo clean

install-dev-deps:
	# rustup install nightly
	rustup override set nightly
	# rustup update nightly
	cargo install clippy
	cargo install rustfmt

lint:
	cargo clippy
	echo Checking for FIXME/TODO...
	! grep --color -En 'FIXME|TODO' src/*.rs
	echo Checking for long lines...
	! grep --color -En '.{101}' src/*.rs

format:
	cargo fmt

release-preps TAG:
	git checkout -b release-{{TAG}}

release TAG:
	@echo "Bumping version numbers"
	./scripts/bump_cargo_version.sh {{TAG}}
	./scripts/bump_release_dockerfile_version.sh {{TAG}}
	@echo "Git operations"
	git commit -a -m "Bump version numbers to {{TAG}}"
	git checkout master
	git merge --no-ff release-{{TAG}}
	git branch -D release-{{TAG}}
	git tag -s -F changelog/{{TAG}}.txt
	git push --follow-tags origin master

release-ci TAG:
	@echo "Docker image release"
	just docker_image_release {{TAG}}
	@echo "Uploading the binary artifact"
	github-release release --tag {{TAG}}
	github-release upload --tag {{TAG}} --name "feeder" --file target/release/feeder

# login into the github docker package registry
_docker_login:
	@docker login docker.pkg.github.com -u ${GITHUB_USERNAME} -p ${GITHUB_TOKEN}

# build the develop docker image
docker-image-develop: test-release _docker_login
	cp config/Rocket.toml docker/Rocket.toml
	cp target/release/feeder docker/feeder
	docker build -t feeder:develop -f docker/Dockerfile-dev ./docker
	docker tag feeder:develop docker.pkg.github.com/${GITHUB_USERNAME}/feeder/feeder:develop
	docker push docker.pkg.github.com/${GITHUB_USERNAME}/feeder/feeder:develop
	./ci/test_docker.sh develop

# build the release docker image, requires the tag
docker_image_release TAG: test-release _docker_login
	cp config/Rocket.toml docker/Rocket.toml
	cp target/release/feeder docker/feeder
	cd docker
	docker build -t feeder:{{TAG}} -f docker/Dockerfile ./docker
	docker tag feeder:{{TAG}} docker.pkg.github.com/${GITHUB_USERNAME}/feeder/feeder:{{TAG}}
	docker push docker.pkg.github.com/${GITHUB_USERNAME}/feeder/feeder:{{TAG}}
	./ci/test_docker.sh {{TAG}}
