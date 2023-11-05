use std::error::Error;
use std::path::Path;

use crate::errors::TestError;
use crate::js::JSEngine;
use crate::wasm::WasmEngine;

pub enum Engine {
    Wasm(WasmEngine),
    JS(JSEngine),
}

pub struct TestResult {
    path: Box<Path>,
    time: u64,
    cpu_time: u64,
    cpu: Vec<u64>,
    mem: Vec<u64>,
    success: bool,
    engine: Engine,
}


pub fn test(path: &Path) -> Result<Vec<TestResult>, Box<dyn Error>> {
    if path.is_dir() {
        test_dir(path)
    } else {
        let res = test_file(path)?;
        Ok(vec![res])
    }
}

fn test_dir(path: &Path) -> Result<Vec<TestResult>, Box<dyn Error>> {
    let mut results = vec![];

    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let res = &mut test_dir(&path)?;
            results.append(res);
        } else {
            let res = test_file(&path);
            if let Ok(res) = res {
                results.push(res);
            } else {
                match res.err().unwrap() {
                    TestError::InvalidFileType => continue,
                    TestError::Other(err) => return Err(err),
                }
            }
        }
    }

    Ok(results)
}

fn test_file(path: &Path) -> Result<TestResult, TestError> {
    todo!()
}