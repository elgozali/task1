
.PHONY: core api web

core:
	cargo run --bin core

api:
	cargo run --bin api

web:
	cargo run --bin web
