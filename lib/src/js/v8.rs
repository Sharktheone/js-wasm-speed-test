use std::path::Path;

use crate::{Engine, TestResult};
use crate::errors::TestError;
use crate::js::{JSEngine, JSRunner};
use crate::js::runner::run;
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
    fn run_js_file<'a>(
        &'a mut self,
        path: &Path,
        validator: &'a Validator,
    ) -> Result<TestResult, TestError> {
        run(path,
            validator,
            Engine::JS(JSEngine::V8),
            |(file, reruns)| {
                use v8::{Context, ContextScope, HandleScope, Isolate};

                let isolate = &mut Isolate::new(Default::default());
                let hs = &mut HandleScope::new(isolate);
                let c = Context::new(hs);
                let s = &mut ContextScope::new(hs, c);

                let code = v8::String::new(s, &file).unwrap();
                let script = v8::Script::compile(s, code, None).unwrap();

                for _ in 0..reruns {
                    script.run(s).unwrap();
                }
            })
    }
}
