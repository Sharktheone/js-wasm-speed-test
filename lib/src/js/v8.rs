use std::{fs, thread};
use std::path::Path;
use std::time::Instant;


use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use v8::{Context, ContextScope, HandleScope, Isolate};

use crate::errors::TestError;
use crate::js::{JSEngine, JSRunner};
use crate::{Engine, ResourceUsage, TestResult};
use crate::validator::Validator;

pub struct V8;

static mut INITIALIZED: bool = false;


impl V8 {
    #[allow(clippy::new_without_default)]
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
    fn run_js_file(&mut self, path: &Path, _validator: &Validator) -> Result<TestResult, TestError> {
        if !path.is_file() {
            return Err(TestError::IsDir);
        }

        if path.extension().unwrap().to_str().unwrap() != "js" {
            return Err(TestError::InvalidFileType);
        }

        let file = fs::read_to_string(path)?;

        let mut sys = System::new_all();
        let mut res = TestResult::new(path, Engine::JS(JSEngine::V8));


        procspawn::init();

        let h = procspawn::spawn(file, |file| {
            let isolate = &mut Isolate::new(Default::default());
            let hs = &mut HandleScope::new(isolate);
            let c = Context::new(hs);
            let s = &mut ContextScope::new(hs, c);

            let code = v8::String::new(s, &file).unwrap();
            let script = v8::Script::compile(s, code, None).unwrap();
            script.run(s).unwrap();

            for _ in 0..1000000 {
                script.run(s).unwrap();
            }

            println!("child: {}", std::process::id());
        });

        let pid = h.pid().unwrap();


        let handle = thread::spawn(move || {
            h.join().unwrap();
        });

        let start = Instant::now();

        let pid = Pid::from_u32(pid);

        loop {
            sys.refresh_process(pid);
            sys.refresh_cpu();
            if let Some(sys) = sys.process(pid) {
                res.resources.push(ResourceUsage {
                    cpu: sys.cpu_usage(),
                    mem: sys.memory(),
                    elapsed: start.elapsed().as_micros(),
                });
                res.cpu_time = sys.start_time();
            }

            //TODO would be nice to get the cpu time... maybe try another crate?

            if handle.is_finished() {
                break;
            }

            thread::sleep(std::time::Duration::from_millis(10));
        }


        Ok(res)
    }
}