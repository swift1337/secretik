test:
	cargo test

build:
	cargo build --release

install:
	cargo install --path .

fmt:
	cargo fmt

lint:
	cargo clippy

.PHONY: test build install fmt lint
