#![allow(unused)]
fn main() {
    #[link(wasm_import_module = "linking2")]
    extern "C" {
        // imports the name `fn_in_linking2` from `linking2`
        /* fn fn_in_linking2(x: i32) -> i32; */
        // rename
        #[link_name = "fn_in_linking2"]
        fn mulit2(x: i32) -> i32;
    }
    
    #[no_mangle]
    pub fn run() {   
        println!("running linking 1, parse {} to linking2", 10);
        unsafe {
            let result = mulit2(10);
            println!("received {} from linking2", result);
        }
    }
}

