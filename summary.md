Okay, here is my summary from the JS Engines. I've skipped `mozjs` now.

So first, these engines were the engines i considered:

- V8
- SpiderMonkey(mozjs)
- JavaScriptCore
- Deno
- ChakraCore
- Duktape
- Hermes
- JerryScript
- MuJS
- Espruino
- Bun
- Worked
- Boa

Now I go over all the engines and describe the experience / why they were not an option.

- Hermes, JerryScript, Espruino, Worked, Hermes, (Bun): No rust crate found => we would need to do it by ourselves.
- Bun: No crates found, and it also has no API.
- MuJS: found bindings, it is actually written in Rust, but it is a very, very small project. It has 63 commits on GitHub from two contributors.
- Duktape: I found bindings for it, but the project was pretty small. I tested the engine and the script the simple test script that run with all other engines, didn't compile. It didn't like the `let`. With Nico, I found out, that Duktape does not Support ES6, which is obviously considered crucial.
- ChakraCore: Found some bindings, but they were not updated for two years. When I added the bindings as a dependency in cargo and pushed the commit to GitHub, I got immediately some security warnings from the Dependabot
- Deno: I found bindings for the Deno Core, but it creates an `Deno` object in the global scope, which is probably not the best idea to have in a browser engine when thinking of security. This crate is also just a wrapper around the `v8` crate, which provides some useful utility functions and marcos.
- JavaScriptCore: I found bindings, but they seemed not finished even though they exist for several years and are widely used. The main problem was to create a function which can be executed from JS. See [this](https://github.com/tauri-apps/javascriptcore-rs/blob/1f7bba0bcf6a3c1cf69bc990b8f746ae06c3c6be/src/auto/value.rs#L66) I also found some smaller bindings. One of them is hosted on the Gnome gitlab, but seems that they are built by the community.
- MozJS: Found bindings. The MozJS they are probably very usable, they are as you all probably know used in servo. But the main problem is, they are fairly complex, low-level and undocumented. Rust analyzer doesn't work well because of the "imports" with the `include!` macro (see my last message). Most of the API is also unsafe, but the main problem was, I didn't manage to create a class or a global `console` object. I stopped trying after several hours reading through the sources. There is also a function `to_v8` to basically use mozjs as it where v8.
- V8: Found bindings, they are also pretty much undocumented and low-level. This was basically the only engine/bindings I managed to create a global `console` object and several methods like `log` with variadic arguments.
- Boa: Boa has bindings, I didn't try them, but I tested the boa `cli`. The main problem is for example `import` statements are unimplemented. They also say, that boa is very experimental.

I think the best option is to use V8. It is a popular engine, it has bindings, it is at leased some doxygen API references, and it is the only engine I managed to create a global `console` object.
(To be fair, for deno I didn't really try because variadic arguments seem to be very hard with denos `#[op2]` macro). 
The only problem is, that the bindings are low-level and undocumented. We should write our own wrapper around some APIs for example like the`#[op2]` macro.


TLDR: Let's use V8; The API is low-level and undocumented, but I managed to do some things.