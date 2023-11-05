use std::fs;
use std::mem::MaybeUninit;
use std::path::Path;
use std::pin::Pin;

use v8::{Context, ContextScope, HandleScope, Isolate, Local, OwnedIsolate};

use crate::errors::TestError;
use crate::js::JSRunner;
use crate::TestResult;
use crate::validator::Validator;

pub struct V8<'a> {
    isolate: OwnedIsolate,
    handle_scope: HandleScope<'a, ()>,
    context: Local<'a, Context>,
    scope: ContextScope<'a, HandleScope<'a>>,
}


impl<'a> V8<'a> {
    pub fn new() -> Pin<Box<V8<'a>>> {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        let mut v8 = MaybeUninit::<V8>::uninit();

        //hopefully this shit won't explode... it does.. shit TODO
        let v8 = unsafe {
            let ptr = v8.as_mut_ptr();

            (*ptr).isolate = Isolate::new(Default::default());
            (*ptr).handle_scope = HandleScope::new(&mut (*ptr).isolate);
            (*ptr).context = Context::new(&mut (*ptr).handle_scope);
            (*ptr).scope = ContextScope::new(&mut (*ptr).handle_scope, (*ptr).context);

            v8.assume_init()
        };

        Box::pin(v8)
    }
}

impl<'a> JSRunner for V8<'a> {
    fn run_js_file(&mut self, path: &Path, _validator: &Validator) -> Result<TestResult, TestError> {
        if !path.is_file() {
            return Err(TestError::IsDir);
        }

        if path.extension().unwrap().to_str().unwrap() != "js" {
            return Err(TestError::InvalidFileType);
        }

        let file = fs::read_to_string(&path)?;

        let code = v8::String::new(&mut self.scope, &file).unwrap();

        let script = v8::Script::compile(&mut self.scope, code, None).unwrap();

        let result = script.run(&mut self.scope).unwrap().to_string(&mut self.scope).unwrap();

        println!("{}", result.to_rust_string_lossy(&mut self.scope));

        Err(TestError::IsFile)
    }
}