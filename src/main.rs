use std::env;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().next().unwrap();

    let path = Path::new(&path);

    let test = lib::test(path)?;

    Ok(())
}
