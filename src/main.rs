use std::error::Error;
use std::path::Path;
use lib::js::JSRunner;

fn main() -> Result<(), Box<dyn Error>> {
    // let path = env::args().nth(1).unwrap();

    let path = Path::new("test.js");

    let test = lib::Test::new();

    let mut v8 = test.javascriptcore()?;

    let validator = Default::default();

    let res = v8.run_js_file(path, &validator)?;

    println!("{:?}", res);



    Ok(())
}
