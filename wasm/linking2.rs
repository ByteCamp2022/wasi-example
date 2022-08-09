#![allow(unused)]
fn main() {
    // no_mangle使函数在最终二进制文件中保留以供其它模块调用
    #[no_mangle]
    pub fn fn_in_linking2(x: i32) -> i32 {
        println!("running linking 2, received {} from linking1", x);
        println!("linking2 will return {} to linking1", x * 2);
        x * 2
    }
}


