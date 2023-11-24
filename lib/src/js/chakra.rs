use std::path::Path;
use crate::errors::TestError;
use crate::js::{JSEngine, JSRunner};
use crate::js::runner::run;
use crate::{Engine, TestResult};
use crate::validator::Validator;

pub struct Chakra;

impl Chakra {
    pub fn new() -> Self {
        Chakra
    }
}

impl Default for Chakra {
    fn default() -> Self {
        Self::new()
    }
}


impl JSRunner for Chakra {
    fn run_js_file<'a>(
        &'a mut self,
        path: &Path,
        validator: &'a Validator,
    ) -> Result<TestResult, TestError> {
        run(path,
            validator,
            Engine::JS(JSEngine::Chakra),
            |(file, reruns)| {
                let mut runtime = chakracore::Runtime::new().unwrap();
                let context = chakracore::Context::new(&mut runtime).unwrap();
                let guard = context.make_current().unwrap();

                for _ in 0..reruns {
                    let _ = chakracore::script::eval(&guard, &file).unwrap();
                }
            })
    }
}