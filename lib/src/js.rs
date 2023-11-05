mod v8;
mod spidermonkey;
mod javascriptcore;
mod deno;
mod chakra;
mod duktape;

enum Engine {
    V8, //https://v8.dev/
    SpiderMonkey, //https://spidermonkey.dev/
    JavaScriptCore, //https://developer.apple.com/documentation/javascriptcore
    Deno, //https://github.com/denoland/deno
    Chakra, //https://github.com/chakra-core/ChakraCore
    Duktape, //https://github.com/svaarala/duktape
    Hermes, //https://github.com/facebook/hermes
    JerryScript, //https://github.com/jerryscript-project/jerryscript
    MuJS, //https://github.com/ccxvii/mujs NOTE: hmm, seems like a very small project
    Espruino, //https://github.com/espruino/Espruino
}