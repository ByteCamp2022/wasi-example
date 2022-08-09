#![allow(unused)]
fn main() {
    #[link(wasm_import_module = "host")]
    extern "C" {
        fn host_fn(param: i32);
    }

    #[no_mangle]
    pub fn run(i: i32) {
        println!("Wasm received: {}", i);
        println!("Passing {} to host_fn...", i * 2);
        unsafe {
            host_fn(i * 2);
        }
    }
}
