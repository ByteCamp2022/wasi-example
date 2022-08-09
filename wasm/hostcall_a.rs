#![allow(unused)]
fn main() {
    #[link(wasm_import_module = "host")]
    extern "C" {
        fn call_b(param: i32) -> i32;
    }

    #[no_mangle]
    pub fn run() {
        println!("hostcall_a is sending {} to b", 233);
        println!("hostcall_a received {} from b", call_b(233));
    }
}
