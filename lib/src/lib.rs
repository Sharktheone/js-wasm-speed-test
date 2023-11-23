#![allow(dead_code)] // shut up

use crate::errors::TestError;
use crate::js::javascriptcore::JavaScriptCore;
use crate::js::v8::V8;
pub use test::*;

mod benchmark;
mod errors;
pub mod js;
mod resources;
mod test;
pub mod validator;
pub mod wasm;

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
