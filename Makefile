build:
	rustc wasm/wasi.rs --target wasm32-wasi
	rustc wasm/wasi2.rs --target wasm32-wasi
	cargo build

run:
	cargo run

clean:
	cargo clean
	rm *.wasm
