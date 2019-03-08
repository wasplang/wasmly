use wasmly::DataType::*;
use wasmly::*;

fn main() -> std::io::Result<()> {
    let mut app = App::new(vec![]);
    let mut main = Function::new();
    main.with_name("main");
    main.with_output(I32);
    main.with_instructions(vec![
        WebAssembly::I32_CONST,
        42.into(), // return 42
        WebAssembly::END,
    ]);
    app.add_function(main);
    app.write_to_file("out.wasm")?;

    Ok(())
}
