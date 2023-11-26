use std::path::Path;

use v8::{Context, ContextScope, HandleScope, Isolate, Local, Object};

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

    fn register_console(s: &mut ContextScope<HandleScope>, global: Local<Object>) {
        let console_key = v8::String::new(s, "console").unwrap();
        let console_val = Object::new(s);


        //Fn<(&mut v8::HandleScope<'s>, v8::FunctionCallbackArguments<'s>, v8::ReturnValue<'_>)>
        let log = v8::FunctionTemplate::new(s, |hs: &mut HandleScope, args: v8::FunctionCallbackArguments, _ret: v8::ReturnValue<'_> | {
            let mut out = String::new();

            for i in 0..args.length() {
                let arg = args.get(i);
                let arg = arg.to_string(hs).unwrap();
                let arg = arg.to_rust_string_lossy(hs);
                out.push_str(&arg);
                out.push(' ');
            }
            out.pop();
            println!("{}", out);
        }).get_function(s).unwrap();
        let log_key = v8::String::new(s, "log").unwrap();
        console_val.set(s, log_key.into(), log.into());

        let warn_key = v8::String::new(s, "warn").unwrap();
        console_val.set(s, warn_key.into(), log.into());

        let error_key = v8::String::new(s, "error").unwrap();
        console_val.set(s, error_key.into(), log.into());

        let info_key = v8::String::new(s, "info").unwrap();
        console_val.set(s, info_key.into(), log.into());

        global.set(s, console_key.into(), console_val.into());
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
                let isolate = &mut Isolate::new(Default::default());
                let hs = &mut HandleScope::new(isolate);
                let c = Context::new(hs);
                let s = &mut ContextScope::new(hs, c);

                let global = c.global(s);

                Self::register_console(s, global);

                let code = v8::String::new(s, &file).unwrap();
                let script = v8::Script::compile(s, code, None).unwrap();

                for _ in 0..reruns {
                    script.run(s).unwrap();
                }
            })
    }
}