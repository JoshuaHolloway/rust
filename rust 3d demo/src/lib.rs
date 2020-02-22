// Don't need to repeat again:
// > rustup target add wasm32-unknown-unknown

// > cargo build --target wasm32-unknown wasm32-unknown

// extern crate wasm_bindgen;
// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// #[wasm_bindgen]
// pub fn say_hellow_from_rust() {
//     log("Hello, from rust");
// }