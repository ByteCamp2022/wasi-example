use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

// 一段最简单的rust调用wasm的example
// WASI还有linker似乎是非必需的
// srds, 按照wasmtime在crate上给的文档 https://docs.rs/wasmtime/0.39.1/wasmtime/index.html
// 第一种方式好像有点问题，最后还是直接用linker了 https://docs.rs/wasmtime/0.39.1/wasmtime/index.html#linking
pub fn basic() -> Result<()> {
    println!("running basic example...");
    
    let engine = Engine::default();
    let module = Module::from_file(&engine, "basic.wasm")?;
    let mut linker = Linker::new(&engine);
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // 模块中会从host模块引入host_fn函数
    linker.func_wrap("host", "host_fn", |_: Caller<'_, WasiCtx>, param: i32| {
        println!("Host received {} from wasm", param);
    })?;

    let instance = linker.instantiate(&mut store, &module)?;
    let func = instance.get_typed_func::<i32, (), _>(&mut store, "run")?;

    println!("Passing 233 to wasm...");
    func.call(&mut store, 233)?;

    println!();
    Ok(())
}