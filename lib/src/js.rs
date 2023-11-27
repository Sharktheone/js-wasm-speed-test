use std::path::Path;

use crate::errors::TestError;
use crate::validator::Validator;
use crate::TestResult;

// pub(crate) mod chakra;
pub(crate) mod deno;
pub(crate) mod duktape;
pub(crate) mod javascriptcore;

#[cfg(feature = "mozjs")]
pub(crate) mod spidermonkey;

mod runner;
#[cfg(feature = "v8")]
pub(crate) mod v8;

#[derive(Debug, Clone)]
pub enum JSEngine {
    #[cfg(feature = "v8")]
    V8,
    //https://v8.dev/
    #[cfg(feature = "mozjs")]
    SpiderMonkey,
    //https://spidermonkey.dev/
    JavaScriptCore,
    //https://developer.apple.com/documentation/javascriptcore
    Deno,
    //https://github.com/denoland/deno
    // Chakra,         //https://github.com/chakra-core/ChakraCore
    Duktape, //https://github.com/svaarala/duktape
             // Hermes, //https://github.com/facebook/hermes
             // JerryScript, //https://github.com/jerryscript-project/jerryscript
             // MuJS, //https://github.com/ccxvii/mujs NOTE: hmm, seems like a very small project
             // Espruino, //https://github.com/espruino/Espruino
}

pub trait JSRunner {
    fn run_js_file<'a>(
        &'a mut self,
        path: &Path,
        validator: &'a Validator,
    ) -> Result<TestResult, TestError>;
}
