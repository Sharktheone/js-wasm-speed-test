use std::env;
use std::path::Path;

fn main() {
    let path = env::args().next().unwrap();

    let path = Path::new(&path);
}
