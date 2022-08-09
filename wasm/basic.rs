#[no_mangle]
pub extern "C" fn run(i: i32) -> i32 {   
    i * 2
}

fn main() {
    println!("Hello, world!");
}