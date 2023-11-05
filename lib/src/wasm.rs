use std::error::Error;
use crate::TestResult;
use crate::validator::Validator;

mod wasmer;
mod wasmtime;
mod wasmedge;
mod wasmi;
mod deno;
mod javascriptcore;
mod spidermonkey;
mod v8;

pub enum WasmEngine {
    Wasmer, //https://github.com/wasmerio/wasmer
    Wasmtime, //https://github.com/bytecodealliance/wasmtime
    Wamr, //https://github.com/bytecodealliance/wasm-micro-runtime
    WasmEdge, //https://github.com/WasmEdge/WasmEdge
    Wasmi, //https://github.com/paritytech/wasmi
    SpiderMonkey, //https://spidermonkey.dev/
    V8, //https://v8.dev/
    JavaScriptCore, //https://developer.apple.com/documentation/javascriptcore
    Deno, //https://github.com/denoland/deno does deno really support wasm?
}


pub trait WasmRunner {

    fn init() -> Result<(), Box<dyn Error>>;
    fn run_wasm_file(&self, path: &str, validator: &Validator) -> Result<TestResult, Box<dyn Error>>;
}