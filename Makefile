build:
	cargo build
	cd examples/rust_float && cargo build
	cd examples/rust_global && cargo build
	cd examples/rust_hello_world && cargo build
	cd examples/rust_simplest && cargo build
setup:
	npm install
