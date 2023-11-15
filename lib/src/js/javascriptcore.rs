use std::fs;
use std::path::Path;
use javascriptcore::{Context, ContextExt};
use crate::errors::TestError;
use crate::js::JSRunner;
use crate::TestResult;
use crate::validator::Validator;

pub struct JavaScriptCore {

}

impl JavaScriptCore {
    pub fn new() -> Self {
        JavaScriptCore {}
    }

}

impl Default for JavaScriptCore {
    fn default() -> Self {
        Self::new()
    }
}

impl JSRunner for JavaScriptCore {
    fn run_js_file(&mut self, path: &Path, validator: &Validator) -> Result<TestResult, TestError> {
        if !path.is_file() {
            return Err(TestError::IsDir);
        }

        if path.extension().unwrap().to_str().unwrap() != "js" {
            return Err(TestError::InvalidFileType);
        }

        let file = fs::read_to_string(path)?;

        procspawn::init();

        let h = procspawn::spawn(file, |file| {
            let context = Context::new();

            for _ in 0..1000000 {
                context.evaluate_script(&file).unwrap();
            }
        });

        h.join().unwrap();


        Err(TestError::IsDir)
    }
}