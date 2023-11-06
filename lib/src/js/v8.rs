use std::{fs, thread};
use std::path::Path;

use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use v8::{Context, ContextScope, HandleScope, Isolate};

use crate::errors::TestError;
use crate::js::JSRunner;
use crate::TestResult;
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

impl<'a> JSRunner for V8 {
    fn run_js_file(&mut self, path: &Path, _validator: &Validator) -> Result<TestResult, TestError> {
        if !path.is_file() {
            return Err(TestError::IsDir);
        }

        if path.extension().unwrap().to_str().unwrap() != "js" {
            return Err(TestError::InvalidFileType);
        }

        let file = fs::read_to_string(path)?;


        procspawn::init();


        let handle = procspawn::spawn(file, |file| {
            let isolate = &mut Isolate::new(Default::default());
            let hs = &mut HandleScope::new(isolate);
            let c = Context::new(hs);
            let s = &mut ContextScope::new(hs, c);

            let code = v8::String::new(s, &file).unwrap();
            let script = v8::Script::compile(s, code, None).unwrap();
            script.run(s).unwrap();

            for _ in 0..100000000 {
                script.run(s).unwrap();
            }

            println!("child: {}", std::process::id());
        });

        let pid = handle.pid().unwrap();

        let mut sys = System::new_all();


        let handle = thread::spawn(move || {
            handle.join().unwrap();
            println!("child exited");
        });

        loop {
            sys.refresh_all();
            if let Some(sys) = sys.process(Pid::from_u32(pid)) {
                println!("=====================");
                println!("cpu: {}", sys.cpu_usage());
                println!("memory: {}", sys.memory());
                println!("=====================");
            }

            if handle.is_finished() {
                break;
            }
        }

        println!("parent: {}", std::process::id());


        Err(TestError::IsFile)
    }
}