use wasmly::DataType::*;
use wasmly::*;

fn main() -> std::io::Result<()> {
    let mut app = App::new(vec![Import::ImportFunction(ImportFunction::new(
        "console_log".to_string(),
        vec![I32],
        None,
    ))]);

    let d = Data::new(0, "Hello World!\0".as_bytes().to_vec());
    app.add_data(d);
    let mut main = Function::new();
    main.with_name("main");
    main.with_instructions(vec![
        WebAssembly::I32_CONST,
        0.into(),
        WebAssembly::CALL,
        0.into(),
        WebAssembly::END,
    ]);
    app.add_function(main);
    app.write_to_file("out.wasm")?;

    Ok(())
}
