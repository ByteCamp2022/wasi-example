#![allow(unused)]
fn main() {
    #[no_mangle]
    pub fn run(i: i32) -> i32 {
        println!("hostcall_b received: {}", i);
        i * 2
    }
}
