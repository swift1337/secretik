test:
	cargo test

build:
	cargo build --release

install:
	cargo install --path .

fmt:
	cargo fmt

.PHONY: test build install fmt
