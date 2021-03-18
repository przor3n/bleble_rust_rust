.PHONY: compile fmt install-fmt fix install-lint lint language-server new-project

compile:
	rustc main.rs

compile-run: compile
	./main

build:
	cargo build

run:
	cargo run

check:
	cargo check

fmt:
	cargo fmt

install-fmt:
	rustup component add rustfmt

fix:
	cargo fix

install-lint:
	rustup component add clippy

lint:
	cargo clippy

language-server:
	rustup component add rls

new-project:
	cargo new --vcs=git ${PROJECT_NAME}

env:
	echo ${PROJECT_NAME}
