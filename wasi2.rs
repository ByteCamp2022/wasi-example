#[no_mangle]
pub extern "C" fn run(i: i32) {   
    println!("running module {}", i);
}

fn main() {
    println!("Hello, world2!");
}


