use std::{fs, thread};
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use crate::resources::ResourceMonitor;
use crate::{Engine, TestResult};
use crate::errors::TestError;
use crate::validator::Validator;


pub(super) fn run(
    path: &Path,
    validator: &Validator,
    engine: Engine,
    run_file: fn((String, u32))
) -> Result<TestResult, TestError>
{
    if !path.is_file() {
        return Err(TestError::IsDir);
    }

    if path.extension().unwrap().to_str().unwrap() != "js" {
        return Err(TestError::InvalidFileType);
    }

    let file = fs::read_to_string(path)?;

    let mut res = TestResult::new(path, engine);
    procspawn::init();

    let reruns = if !validator.http.is_empty() {
        1
    } else {
        validator.reruns
    };

    let mut h = procspawn::spawn((file, reruns), run_file);

    let start = Instant::now();
    let pid = h.pid().unwrap();

    let monitor = ResourceMonitor::new(pid);
    let monitor = Arc::new(monitor);

    let handle = {
        let monitor = Arc::clone(&monitor);
        thread::spawn(move || {
            monitor.start(&start);
        })
    };

    if !validator.http.is_empty() {
        let monitor = Arc::clone(&monitor);
        let http_res = validator.validate_http(&monitor)?;
        h.kill().unwrap();
        res.http = Some(http_res);
    } else {
        h.join().unwrap();
    }

    let monitor = Arc::clone(&monitor);
    monitor.stop(); //hopefully we can lock this shit, while the thread is obviously running... Else it will explode...


    handle.join().unwrap();

    // res.resources = monitor.resources.clone();

    Ok(res)
}