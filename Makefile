help: ## List of commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

test: ## Run tests
	cargo test

build: ## Build the binary
	cargo build --release

install: ## Install the binary
	cargo install --path .

fmt: ## Format the code
	cargo fmt

lint: ## Lint the code
	cargo clippy

publish: ## Publish to crates.io (make publish TAG=vX.Y.Z)
	cargo publish --dry-run
	cargo publish

.PHONY: help test build install fmt lint publish
