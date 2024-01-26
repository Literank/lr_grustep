.PHONY: run test lint clean

run:
	cargo run -- -rn result .

test:
	cargo test

lint:
	cargo clippy

install:
	cargo install --path .

build:
	cargo build --release

clean:
	rm -rf target/
