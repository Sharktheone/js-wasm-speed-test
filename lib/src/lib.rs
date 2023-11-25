#![allow(dead_code)] // shut up

use crate::errors::TestError;
use crate::js::javascriptcore::JavaScriptCore;
use crate::js::v8::V8;
pub use test::*;
use crate::js::{JSRunner};
use crate::js::deno::Deno;
use crate::js::duktape::Duktape;
use crate::js::spidermonkey::SpiderMonkey;

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

    pub fn v8(&self) -> Result<Box<dyn JSRunner>, TestError> {
        Ok(Box::new(V8::new()?))
    }

    pub fn javascriptcore(&self) -> Result<Box<dyn JSRunner>, TestError> {
        Ok(Box::new(JavaScriptCore::new()))
    }

    pub fn deno(&self) -> Result<Box<dyn JSRunner>, TestError> {
        Ok(Box::new(Deno::new()))
    }

    pub fn spidermonkey(&self) -> Result<Box<dyn JSRunner>, TestError> {
       Ok(Box::new(SpiderMonkey::new()))
    }

    pub fn duktape(&self) -> Result<Box<dyn JSRunner>, TestError> {
        Ok(Box::new(Duktape::new()))
    }



}

impl Default for Test {
    fn default() -> Self {
        Self::new()
    }
}
