use std::path::Path;

use javascriptcore::{Class, Context, ContextExt, Value, ValueExt};

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

    // fn register_console(c: &mut Context) {
    //
    //     let console = Value::builder().context(c).build();
    //
    //     Value::is_function()
    //
    //     c.set_value("console", &console);
    //
    // }
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

                // Self::register_console(&mut context); adding functions to values is not supported in the javascriptcore-rs bindings

                for _ in 0..reruns {
                    context.evaluate(&file).unwrap();
                }
            })
    }
}
