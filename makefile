.PHONY: run-dynamic run-static build build-p clean update cu
.DEFAULT_GOAL := run-dynamic

run-dynamic:
	cargo +nightly run --features bevy/dynamic_linking

run-static:
	cargo run

build-p:
	cargo build --release

build:
	cargo build

clean:
	cargo clean

update:
	cargo update

cu: clean update