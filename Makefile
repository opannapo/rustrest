.PHONY: run-with-log run-bin-http run-bin-migration-up run-bin-migration-new

run-with-log:
	UST_LOG=debug cargo run

run-bin-api:
	cargo run --bin api

run-bin-migration-up:
	cargo run --bin migration up

run-bin-migration-new:
	cargo run --bin migration new ${filename}

build-bin-api-release:
	cargo build --release --bin api