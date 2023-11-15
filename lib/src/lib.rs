#![allow(dead_code)] // shut up


pub use test::*;
use crate::errors::TestError;
use crate::js::javascriptcore::JavaScriptCore;
use crate::js::v8::V8;

mod test;
mod errors;
pub mod js;
pub mod wasm;
pub mod validator;
mod benchmark;


pub struct Test;

impl Test {
    pub fn new() -> Self {
        Test
    }

    pub fn v8(&self) -> Result<V8, TestError> {
        V8::new()
    }

    pub fn javascriptcore(&self) -> Result<JavaScriptCore, TestError> {
        Ok(JavaScriptCore::new())
    }
}


impl Default for Test {
    fn default() -> Self {
        Self::new()
    }
}
