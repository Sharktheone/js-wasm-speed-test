use ::std::path::Path;
use ::std::ptr;

use mozjs::jsval::UndefinedValue;
use mozjs::rooted;
use mozjs::rust::{RealmOptions, Runtime};
use mozjs::rust::SIMPLE_GLOBAL_CLASS;
use mozjs::rust::jsapi_wrapped as jsapi;
use mozjs::jsapi::*;

use crate::{Engine, TestResult};
use crate::errors::TestError;
use crate::js::{JSEngine, JSRunner};
use crate::js::runner::run;
use crate::validator::Validator;

pub struct SpiderMonkey;

impl SpiderMonkey {
    pub fn new() -> Self {
        SpiderMonkey
    }
}

impl Default for SpiderMonkey {
    fn default() -> Self {
        Self::new()
    }
}

impl JSRunner for SpiderMonkey {
    fn run_js_file<'a>(
        &'a mut self,
        path: &Path,
        validator: &'a Validator,
    ) -> ::core::result::Result<TestResult, TestError> {
        run(
            path,
            validator,
            Engine::JS(JSEngine::SpiderMonkey),
            |(file, reruns)| {
                let engine = mozjs::rust::JSEngine::init().unwrap();
                let rt = Runtime::new(engine.handle());

                let options = RealmOptions::default();

                rooted!(in(rt.cx()) let global  = unsafe {
                    JS_NewGlobalObject(rt.cx(), &SIMPLE_GLOBAL_CLASS, ptr::null_mut(),
                        OnNewGlobalHookOption::FireOnNewGlobalHook,
                        &*options)
                });


                rooted!(in(rt.cx()) let mut rval = UndefinedValue());


                for _ in 0..reruns {
                    let _ = rt.evaluate_script(
                        global.handle(),
                        &file,
                        "inline.js",
                        1,
                        rval.handle_mut(),
                    );
                }
            },
        )
    }
}