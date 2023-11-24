use std::path::Path;

use javascriptcore::{Context, ContextExt};

use crate::{Engine, TestResult};
use crate::errors::TestError;
use crate::js::{JSEngine, JSRunner};
use crate::js::runner::run;
use crate::validator::Validator;

pub struct JavaScriptCore;

impl JavaScriptCore {
    pub fn new() -> Self {
        JavaScriptCore
    }
}

impl Default for JavaScriptCore {
    fn default() -> Self {
        Self::new()
    }
}

impl JSRunner for JavaScriptCore {
    fn run_js_file<'a>(
        &'a mut self,
        path: &Path,
        validator: &'a Validator,
    ) -> Result<TestResult, TestError> {
        run(path,
            validator,
            Engine::JS(JSEngine::JavaScriptCore),
            |(file, reruns)| {
                let context = Context::new();
                for _ in 0..reruns {
                    context.evaluate(&file).unwrap();
                }
            })
    }
}
