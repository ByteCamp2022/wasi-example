use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};
// struct MyState {
//     counter: i32,
// }

// 一段最简单的rust调用wasm的example
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
    linker.func_wrap("host", "host_fn", |_: Caller<'_, WasiCtx>, param: i32| {
        println!("Host received {} from wasm", param);
    })?;
    

    let instance = linker.instantiate(&mut store, &module)?;
    let func = instance.get_typed_func::<i32, (), _>(&mut store, "run")?;

    println!("Calling wasi");
    func.call(&mut store, 233)?;

    println!();
    Ok(())
}