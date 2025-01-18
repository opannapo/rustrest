.PHONY: run-with-log run-bin-api run-bin-migration-up

run-with-log:
	UST_LOG=debug cargo run

run-bin-api:
	cargo run --bin api

run-bin-migration-up:
	cargo run --bin migration up