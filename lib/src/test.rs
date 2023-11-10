use std::path::Path;

use crate::errors::TestError;
use crate::js::JSEngine;
use crate::wasm::WasmEngine;

#[derive(Debug)]
pub enum Engine {
    Wasm(WasmEngine),
    JS(JSEngine),
}

#[derive(Debug)]
pub struct TestResult {
    pub path: Box<Path>,
    pub time: u64,
    pub cpu_time: u64,
    pub resources: Vec<ResourceUsage>,
    pub success: bool,
    pub engine: Engine,
}

#[derive(Debug)]
pub struct ResourceUsage {
    pub cpu: f32,
    pub mem: u64,
    pub elapsed: u128,
}


impl TestResult {
    pub fn new(path: &Path, engine: Engine) -> Self {
        TestResult {
            path: Box::from(path),
            time: 0,
            cpu_time: 0,
            resources: vec![],
            success: false,
            engine,
        }
    }

}


pub fn test(path: &Path) -> Result<Vec<TestResult>, TestError> {
    if path.is_dir() {
        test_dir(path)
    } else {
        let res = test_file(path)?;
        Ok(vec![res])
    }
}

fn test_dir(path: &Path) -> Result<Vec<TestResult>, TestError> {
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
                    TestError::Other(err) => return Err(TestError::Other(err)),
                    TestError::AlreadyInitialized => return Err(TestError::AlreadyInitialized),
                    _ => continue,
                }
            }
        }
    }

    Ok(results)
}

fn test_file(path: &Path) -> Result<TestResult, TestError> {
    todo!()
}