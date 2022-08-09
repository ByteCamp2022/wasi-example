use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

// 实际上引入多个modules并不麻烦，这段代码主要是引入多个modules，实例化之后用vector存储了一下
pub fn multi_modules() -> Result<()> {
    println!("running multi modules example...");

    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created
    let mut instances: Vec<Instance> = Vec::new();
    let wasm_files = ["wasi.wasm", "wasi2.wasm"];

    for file in wasm_files.iter() {
        println!("Loading {}", file);
        let module = Module::from_file(&engine, file)?;
        // linker.module(&mut store, "", &module)?;
        // linker
        //     .get_default(&mut store, "")?
        //     .typed::<(), (), _>(&store)?
        //     .call(&mut store, ())?;
        let instance = linker.instantiate(&mut store, &module)?;
        instances.push(instance);
    }

    // Run the instances
    let mut index = 1;
    for instance in instances.iter() {
        let f = instance.get_typed_func::<i32, (), _>(&mut store, "run")?;
        println!("Calling export...{}", index);
        f.call(&mut store, index)?;
        index += 1;
    }

    println!();
    Ok(())
}
