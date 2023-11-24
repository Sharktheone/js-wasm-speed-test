use std::path::Path;
use crate::errors::TestError;
use crate::js::{JSEngine, JSRunner};
use crate::js::runner::run;
use crate::{Engine, TestResult};
use crate::validator::Validator;
use kg_js::JsEngine;

pub struct Duktape;

impl Duktape {
    pub fn new() -> Self {
        Duktape
    }
}

impl Default for Duktape {
    fn default() -> Self {
        Self::new()
    }
}

impl JSRunner for Duktape {
    fn run_js_file<'a>(
        &'a mut self,
        path: &Path,
        validator: &'a Validator,
    ) -> Result<TestResult, TestError> {
        run(path,
            validator,
            Engine::JS(JSEngine::Duktape),
            |(file, reruns)| {
                let mut engine = JsEngine::new();

                for _ in 0..reruns {
                    engine.eval(&file).unwrap();
                }
            })
    }
}