#!/usr/bin/env just --justfile

GIT_TAG := `git describe --abbrev=0 --tags`

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

release:
	@echo "Bumping version numbers to {{GIT_TAG}}"
	./scripts/bump_cargo_version.sh {{GIT_TAG}}
	./scripts/bump_release_dockerfile_version.sh {{GIT_TAG}}
	@echo "Git operations"
	git commit -a -m "Bump version numbers to {{GIT_TAG}}"
	git checkout master
	git merge --no-ff release-{{GIT_TAG}}
	git tag -s -F changelog/{{GIT_TAG}}.txt
	git push --follow-tags origin master
	git checkout develop
	git merge --no-ff release-{{GIT_TAG}}
	git branch -D release-{{GIT_TAG}}

release-ci:
	@echo "Uploading the binary artifact for tag {{GIT_TAG}}"
	github-release release --tag {{GIT_TAG}}
	github-release upload --tag {{GIT_TAG}} --name "feeder" --file target/release/feeder
	@echo "Docker image release for tag {{GIT_TAG}}"
	just docker_image_release {{GIT_TAG}}

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
docker_image_release: test-release _docker_login
	cp config/Rocket.toml docker/Rocket.toml
	cp target/release/feeder docker/feeder
	cd docker
	docker build -t feeder:{{GIT_TAG}} -f docker/Dockerfile ./docker
	docker tag feeder:{{GIT_TAG}} docker.pkg.github.com/${GITHUB_USERNAME}/feeder/feeder:{{GIT_TAG}}
	docker push docker.pkg.github.com/${GITHUB_USERNAME}/feeder/feeder:{{GIT_TAG}}
	./ci/test_docker.sh {{GIT_TAG}}
