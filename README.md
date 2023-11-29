# JS/WASM runtime speed test

This project is created in the context of [GoSuB Engine](https://github.com/gosub-browser/gosub-engine) to decide which
runtime to use for the engine.

## Tested Engines:

- JS
    - [V8](https://v8.dev/)
    - [SpiderMonkey](https://developer.mozilla.org/en-US/docs/Mozilla/Projects/SpiderMonkey)
    - [JavaScriptCore](https://developer.apple.com/documentation/javascriptcore)
    - [Deno](https://github.com/denoland/deno)
    - [ChakraCore](https://developer.apple.com/documentation/javascriptcore)
    - [Duktape](https://github.com/svaarala/duktape)
    - [Hermes](https://github.com/facebook/hermes)
    - [JerryScript](https://github.com/jerryscript-project/jerryscript)
    - ([MuJS](https://github.com/ccxvii/mujs)) NOTE: seems like a very small project
    - [Espruino](https://github.com/espruino/Espruino)
    - [Bun](https://github.com/oven-sh/bun)
    - [Worked](https://github.com/cloudflare/workerd)
    - [Boa](https://github.com/boa-dev/boa)
- WASM
    - [V8](https://v8.dev/)
    - [SpiderMonkey](https://developer.mozilla.org/en-US/docs/Mozilla/Projects/SpiderMonkey)
    - [JavaScriptCore](https://developer.apple.com/documentation/javascriptcore)
    - [Deno](https://github.com/denoland/deno)?
    - [Wasmer](https://github.com/wasmerio/wasmer)
    - [Wasmtime](https://github.com/bytecodealliance/wasmtime)
    - [Wamr](https://github.com/bytecodealliance/wasm-micro-runtime)
    - [WasmEdge](https://github.com/WasmEdge/WasmEdge)
    - [Wasmi](https://github.com/paritytech/wasmi)

> **NOTE**
> - Due to popularity MuJS should probably not be used for the GoSuB Engine
> - Bun has no API
> - No crates found for Hermes, JerryScript, Espruino
> - Duktape has no popular crate for rust
> - Wasmr has only a crate which is last updated 3 years ago, but it has go bindings => not preferred

## Results

Speed has not really been tested, but for the results of this test, see [summary.md](summary.md) (copied message I wrote on the GoSuB Zulip Chat)

