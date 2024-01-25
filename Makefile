.PHONY: run test clean

run:
	cargo run -- -rn result .

test:
	cargo test

install:
	cargo install --path .

build:
	cargo build --release

clean:
	rm -rf target/
