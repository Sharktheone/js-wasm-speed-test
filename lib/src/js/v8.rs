use std::fs;
use std::io::Error;
use std::path::Path;

use v8::{Context, ContextScope, HandleScope, Isolate};

use crate::errors::TestError;
use crate::js::JSRunner;
use crate::TestResult;
use crate::validator::Validator;

// pub struct V8<'a> {
//     isolate: Pin<Box<OwnedIsolate>>,
//     handle_scope: Pin<Box<HandleScope<'a, ()>>>,
//     context: Pin<Box<Local<'a, Context>>>,
//     // scope: Pin<Box<ContextScope<'a, HandleScope<'a>>>>,
//     _pin: std::marker::PhantomPinned,
// }

pub struct V8;

static mut INITIALIZED: bool = false;



impl V8 {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Result<V8, TestError> {

        if unsafe { INITIALIZED } {
            return Err(TestError::AlreadyInitialized);
        }

        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        unsafe {
            INITIALIZED = true;
        }


        //     Box::pin(V8 {
        //         isolate,
        //         handle_scope,
        //         context,
        //         // scope,
        //         _pin: std::marker::PhantomPinned,
        //     })

        Ok(V8)
    }

    // #[inline]
    // fn s(&mut self) -> &mut ContextScope<'a, HandleScope<'a>> {
    //     // self.scope.as_mut().get_mut()
    // }
}

impl Drop for V8 {
    fn drop(&mut self) {
        unsafe {
            v8::V8::dispose();
        }

        v8::V8::dispose_platform();
    }
}

impl<'a> JSRunner for V8 {
    fn run_js_file(&mut self, path: &Path, _validator: &Validator) -> Result<TestResult, TestError> {
        if !path.is_file() {
            return Err(TestError::IsDir);
        }

        if path.extension().unwrap().to_str().unwrap() != "js" {
            return Err(TestError::InvalidFileType);
        }

        let file = fs::read_to_string(path)?;

        let isolate = &mut Isolate::new(Default::default());
        let hs = &mut HandleScope::new(isolate);
        let c = Context::new(hs);
        let s = &mut ContextScope::new(hs, c); //TODO: Would be nice if we could embed these values into the struct


        let code = v8::String::new(s, &file).unwrap();

        let script = v8::Script::compile(s, code, None).unwrap();

        let result = script.run(s).unwrap().to_string(s).unwrap();

        println!("{}", result.to_rust_string_lossy(s));


        Err(TestError::IsFile)
    }
}