.PHONY: test build clippy fmt

test:
	cargo test

build:
	cargo build --release

clippy:
	cargo clippy -- -D warnings

fmt:
	cargo fmt
