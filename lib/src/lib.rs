#![allow(dead_code)] // shut up

pub use test::*;

use crate::errors::TestError;
use crate::js::deno::Deno;
use crate::js::duktape::Duktape;
use crate::js::javascriptcore::JavaScriptCore;
use crate::js::JSRunner;

mod benchmark;
mod errors;
pub mod js;
mod resources;
mod test;
pub mod validator;
pub mod wasm;

pub struct Test;

#[cfg(all(feature = "mozjs", feature = "v8"))]
compile_error!("Features `mozjs` and `v8` are mutually exclusive and cannot be enabled at the same time.");


impl Test {
    pub fn new() -> Self {
        Test
    }

    pub fn v8(&self) -> Result<Box<dyn JSRunner>, TestError> {
        #[cfg(feature = "v8")]
        {
            use crate::js::v8::V8;
            Ok(Box::new(V8::new()?))
        }
        #[cfg(not(feature = "v8"))]
        Err(TestError::FeatureNotEnabled("v8"))
    }

    pub fn javascriptcore(&self) -> Result<Box<dyn JSRunner>, TestError> {
        Ok(Box::new(JavaScriptCore::new()))
    }

    pub fn deno(&self) -> Result<Box<dyn JSRunner>, TestError> {
        Ok(Box::new(Deno::new()))
    }

    pub fn spidermonkey(&self) -> Result<Box<dyn JSRunner>, TestError> {
        #[cfg(feature = "mozjs")]
        {
            use crate::js::spidermonkey::SpiderMonkey;
            Ok(Box::new(SpiderMonkey::new()))
        }
        #[cfg(not(feature = "mozjs"))]
        Err(TestError::FeatureNotEnabled("mozjs"))
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
