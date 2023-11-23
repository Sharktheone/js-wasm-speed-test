use std::{fs, thread};
use std::cell::RefCell;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

use v8::{Context, ContextScope, HandleScope, Isolate};

use crate::{Engine, TestResult};
use crate::errors::TestError;
use crate::js::{JSEngine, JSRunner};
use crate::resources::ResourceMonitor;
use crate::validator::Validator;

pub struct V8;

static mut INITIALIZED: bool = false;


impl V8 {
    pub fn new() -> Result<V8, TestError> {
        if unsafe { INITIALIZED } {
            return Err(TestError::AlreadyInitialized);
        }

        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        unsafe {
            INITIALIZED = true;
        }


        Ok(V8)
    }
}

impl Drop for V8 {
    fn drop(&mut self) {
        unsafe {
            v8::V8::dispose();
        }

        v8::V8::dispose_platform();
    }
}

impl JSRunner for V8 {
    fn run_js_file<'a>(&'a mut self, path: &Path, validator: &'a Validator) -> Result<TestResult, TestError> {
        if !path.is_file() {
            return Err(TestError::IsDir);
        }

        if path.extension().unwrap().to_str().unwrap() != "js" {
            return Err(TestError::InvalidFileType);
        }

        let file = fs::read_to_string(path)?;

        let mut res = TestResult::new(path, Engine::JS(JSEngine::V8));
        procspawn::init();

        let reruns = if !validator.http.is_empty() {
            1
        } else {
            validator.reruns
        };

        let mut h = procspawn::spawn((file, reruns), |(file, reruns)| {
            let isolate = &mut Isolate::new(Default::default());
            let hs = &mut HandleScope::new(isolate);
            let c = Context::new(hs);
            let s = &mut ContextScope::new(hs, c);

            let code = v8::String::new(s, &file).unwrap();
            let script = v8::Script::compile(s, code, None).unwrap();

            for _ in 0..reruns {
                script.run(s).unwrap();
            }
        });

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

        println!("stopped");

        handle.join().unwrap();

        // res.resources = monitor.resources.clone();


        Ok(res)
    }
}