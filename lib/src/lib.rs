#![allow(dead_code)] // shut up


pub use test::*;
use crate::errors::TestError;
use crate::js::v8::V8;

mod test;
mod errors;
pub mod js;
pub mod wasm;
pub mod validator;


pub struct Test;

impl Test {
    pub fn new() -> Self {
        Test
    }

    pub fn v8(&self) -> Result<V8, TestError> {
        V8::new()
    }
}


impl Default for Test {
    fn default() -> Self {
        Self::new()
    }
}