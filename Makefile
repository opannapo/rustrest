.PHONY: run-with-log run-bin-api

run-with-log:
	UST_LOG=debug cargo run

run-bin-api:
	cargo run --bin api