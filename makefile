.PHONY: run-dynamic run-static build build-p clean update cu open
.DEFAULT_GOAL := run-dynamic

run-dynamic:
	cargo +nightly run --features bevy/dynamic_linking

run-debug:
	cargo +nightly run --features bevy/dynamic_linking,vibrant/debug

run-editor:
	cargo +nightly run --features bevy/dynamic_linking,vibrant/editor

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

open:
	C:\Program Files\Blender Foundation\Blender 4.0\blender-launcher.exe