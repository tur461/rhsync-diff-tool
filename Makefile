.DEFAULT_GOAL := build

build:
	clear && cargo build --release

test:
	clear && RUST_TEST_TASKS=1 cargo test

