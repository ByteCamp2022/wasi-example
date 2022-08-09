//! Example of instantiating two modules which link to each other.

// You can execute this example with `cargo run --example linking`

use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

// wasmtime文档中抄的例字，用于模块间相互调用的方式
// https://docs.wasmtime.dev/examples-c-linking.html
pub fn linking_modules() -> Result<()> {
    println!("running linking modules example...");

    let engine = Engine::default();

    // First set up our linker which is going to be linking modules together. We
    // want our linker to have wasi available, so we set that up here as well.
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // Load and compile our two modules
    println!("Loading linking1.wasm");
    let linking1 = Module::from_file(&engine, "linking1.wasm")?;
    println!("Loading linking2.wasm");
    let linking2 = Module::from_file(&engine, "linking2.wasm")?;

    // Configure WASI and insert it into a `Store`
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our first module which only uses WASI, then register that
    // instance with the linker since the next linking will use it.
    println!("instantiating linking2");
    let linking2 = linker.instantiate(&mut store, &linking2)?;
    println!("registering linking2");
    linker.instance(&mut store, "linking2", linking2)?;

    // And with that we can perform the final link and the execute the module.
    println!("instantiating linking1");
    let linking1 = linker.instantiate(&mut store, &linking1)?;
    println!("calling linking1 which will call linking2 in it");
    let run = linking1.get_typed_func::<(), (), _>(&mut store, "run")?;
    run.call(&mut store, ())?;

    println!();
    Ok(())
}
