use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

// 这段代码首先载入模块hostcall_a.wasm, hostcall_b.wasm并实例化
// 执行hostcall_a.wasm，其中a会向host请求调用hostcall_b.wasm中的方法
pub fn hostcall() -> Result<()> {
    println!("running hostcall example...");
    
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    println!("loading hostcall_b.wasm...");
    let module = Module::from_file(&engine, "hostcall_b.wasm")?;
    let instance_b = linker.instantiate(&mut store, &module)?;
    // 获取b模块中的方法run
    let func_b = instance_b.get_typed_func::<i32, i32, _>(&mut store, "run")?;
    
    // 模块a会通过host请求调用b模块中的方法
    println!("loading hostcall_a.wasm...");
    let module = Module::from_file(&engine, "hostcall_a.wasm")?;
    // 首先将b的方法绑定到a的imports中
    linker.func_wrap("host", "call_b", |_: Caller<'_, WasiCtx>, param: i32| -> i32 {
        println!("Host received {} from a", param);
        println!("Host is calling b...");
        let result = func_b.call(&mut store, param);
        result
    })?;
    linker.func_wrap("host", "call_b", func_b)?;

    let instance_a = linker.instantiate(&mut store, &module)?;    
    let func_a = instance_a.get_typed_func::<(), (), _>(&mut store, "run")?;

    println!("running hostcall_a.wasm...");
    func_a.call(&mut store, ())?;

    println!();
    Ok(())
}