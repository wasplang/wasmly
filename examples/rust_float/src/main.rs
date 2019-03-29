use wasmly::DataType::*;
use wasmly::*;

fn main() -> std::io::Result<()> {
    let mut app = App::new(vec![]);
    let mut main = Function::new();
    main.with_name("main");
    main.with_output(F64);
    main.with_instructions(vec![
        WebAssembly::F64_CONST,
        420.5.into(), // return 42
        WebAssembly::END,
    ]);
    app.add_function(main);
    app.write_to_file("out.wasm")?;

    Ok(())
}
