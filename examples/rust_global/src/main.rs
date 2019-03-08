use wasmly::DataType::*;
use wasmly::*;

fn main() -> std::io::Result<()> {
    let mut app = App::new(vec![]);
    let mut main = Function::new();
    main.with_name("main");
    main.with_output(I32);
    main.with_instructions(vec![WebAssembly::GLOBAL_GET, 0.into(), WebAssembly::END]);
    app.add_function(main);
    app.add_global(Global::new(33, false));
    app.write_to_file("out.wasm")?;

    Ok(())
}
