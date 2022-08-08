build:
	rustc wasi.rs --target wasm32-wasi
	rustc wasi2.rs --target wasm32-wasi
	cargo build

run:
	cargo run

clean:
	cargo clean
	rm *.wasm
