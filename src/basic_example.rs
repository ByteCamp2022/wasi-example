use anyhow::Result;
use wasmtime::*;

struct MyState {
    counter: i32,
}

// 一段最简单的rust调用wasm的example
pub fn basic() -> Result<()> {
    println!("running basic example...");
    
    let engine = Engine::default();
    let module = Module::from_file(&engine, "basic.wasm")?;
    let mut store = Store::new(&engine, MyState { counter: 0 });

    let host_fn = Func::wrap(&mut store, |caller: Caller<'_, MyState>, param: i32| {
        println!("Host received {} from wasm", param);
        println!("My host counter is: {}", caller.data().counter);
    });

    let imports = [host_fn.into()];
    let instance = Instance::new(&mut store, &module, &imports)?;
    let func = instance.get_typed_func::<i32, (), _>(&mut store, "run")?;

    println!("Calling wasi");
    func.call(&mut store, 233)?;

    println!();
    Ok(())
}