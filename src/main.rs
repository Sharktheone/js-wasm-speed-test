use std::env;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).unwrap();
    let engine = env::args().nth(2).unwrap();
    let path = Path::new(&path);

    let test = lib::Test::new();

    if path.extension().unwrap().eq("js") {
        let mut engine = match engine.to_lowercase().as_str() {
            "duktape" | "dt" => test.duktape()?,
            "javascriptcore" | "jsc" => test.javascriptcore()?,
            "v8" => test.v8()?,
            "deno" => test.deno()?,
            "spidermonkey" | "sm" | "mozjs" => test.spidermonkey()?,
            _ => return Err(Box::from("Unknown engine")),
        };

        let validator = Default::default();

        let res = engine.run_js_file(path, &validator)?;

        println!("{:?}", res);

    } else {
        return Err(Box::from("Not a JS file; WASM not supported yet"));
    }

    Ok(())
}
