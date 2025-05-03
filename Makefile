test:
	cargo test

build:
	cargo build --release

install:
	cargo install --path .

.PHONY: test build install
