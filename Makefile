.PHONY: run-with-log run-bin-http run-bin-migration-up run-bin-migration-new

run-with-log:
	UST_LOG=debug cargo run

run-bin-http:
	cargo run --bin http

run-bin-migration-up:
	cargo run --bin migration up

run-bin-migration-new:
	cargo run --bin migration new ${filename}

build-bin-http:
	cargo build --release --bin http