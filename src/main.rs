use std::env;
use std::error::Error;
use std::path::Path;
use lib::js::JSRunner;

fn main() -> Result<(), Box<dyn Error>> {
    // let path = env::args().nth(1).unwrap();

    let path = Path::new("test.js");

    let mut v8 = lib::js::V8::new()?;

    let validator = lib::validator::Validator::new();

    let _ = v8.run_js_file(path, &validator)?;

    Ok(())
}
