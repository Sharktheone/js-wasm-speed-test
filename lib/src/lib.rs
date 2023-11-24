#![allow(dead_code)] // shut up

use crate::errors::TestError;
use crate::js::javascriptcore::JavaScriptCore;
use crate::js::v8::V8;
pub use test::*;
use crate::js::{JSRunner};
use crate::js::chakra::Chakra;
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

    pub fn v8(&self) -> Result<&impl JSRunner, TestError> {
        Ok(&V8::new()?)
    }

    pub fn javascriptcore(&self) -> Result<&impl JSRunner, TestError> {
        Ok(&JavaScriptCore::new())
    }

    pub fn deno(&self) -> Result<&impl JSRunner, TestError> {
        Ok(&Deno::new())
    }

    pub fn spidermonkey(&self) -> Result<&impl JSRunner, TestError> {
       Ok(&SpiderMonkey::new())
    }

    pub fn chakra(&self) -> Result<&impl JSRunner, TestError> {
        Ok(&Chakra::new())
    }

    pub fn duktape(&self) -> Result<&impl JSRunner, TestError> {
        Ok(&Duktape::new())
    }



}

impl Default for Test {
    fn default() -> Self {
        Self::new()
    }
}
