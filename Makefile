build:
	rustc wasm/basic.rs --target wasm32-wasi
	rustc wasm/wasi.rs --target wasm32-wasi
	rustc wasm/wasi2.rs --target wasm32-wasi
	rustc wasm/linking1.rs --target wasm32-wasi
	rustc wasm/linking2.rs --target wasm32-wasi
	rustc wasm/hostcall_b.rs --target wasm32-wasi
	cargo build

run:
	cargo run

clean:
	cargo clean
	rm *.wasm
