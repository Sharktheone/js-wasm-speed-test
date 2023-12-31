use deno_core::{op2, JsRuntime, ModuleCode};
use std::path::Path;

use crate::errors::TestError;
use crate::js::runner::run;
use crate::js::{JSEngine, JSRunner};
use crate::validator::Validator;
use crate::{Engine, TestResult};

pub struct Deno;

impl Deno {
    pub fn new() -> Self {
        Deno
    }
}
impl Default for Deno {
    fn default() -> Self {
        Self::new()
    }
}

impl JSRunner for Deno {
    fn run_js_file<'a>(
        &'a mut self,
        path: &Path,
        validator: &'a Validator,
    ) -> Result<TestResult, TestError> {
        run(
            path,
            validator,
            Engine::JS(JSEngine::Deno),
            |(file, reruns)| {
                let mut runtime = JsRuntime::new(Default::default());
                for _ in 0..reruns {
                    let code = ModuleCode::from(file.clone());
                    let _ = runtime.execute_script("test", code).unwrap();
                }
            },
        )
    }
}
