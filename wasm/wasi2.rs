#![allow(unused)]
fn main() {
    #[no_mangle]
    pub extern "C" fn run(i: i32) {   
        println!("2. running module {}", i);
    }
}

