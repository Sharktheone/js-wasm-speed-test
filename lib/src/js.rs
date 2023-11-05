use std::error::Error;
use std::path::Path;
use crate::errors::TestError;
use crate::TestResult;
use crate::validator::Validator;

mod v8;
mod spidermonkey;
mod javascriptcore;
mod deno;
mod chakra;
mod duktape;

pub use v8::*;
pub use spidermonkey::*;
pub use javascriptcore::*;
pub use deno::*;
pub use chakra::*;
pub use duktape::*;

pub enum JSEngine {
    V8, //https://v8.dev/
    SpiderMonkey, //https://spidermonkey.dev/
    JavaScriptCore, //https://developer.apple.com/documentation/javascriptcore
    Deno, //https://github.com/denoland/deno
    Chakra, //https://github.com/chakra-core/ChakraCore
    Duktape, //https://github.com/svaarala/duktape
    // Hermes, //https://github.com/facebook/hermes
    // JerryScript, //https://github.com/jerryscript-project/jerryscript
    // MuJS, //https://github.com/ccxvii/mujs NOTE: hmm, seems like a very small project
    // Espruino, //https://github.com/espruino/Espruino
}


pub trait JSRunner {
    fn run_js_file(&mut self, path: &Path, validator: &Validator) -> Result<TestResult, TestError>;
}