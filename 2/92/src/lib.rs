// > npm run dev
// > npm run build

// Don't need to repeat again:
// > rustup target add wasm32-unknown-unknown

// > cargo build --target wasm32-unknown-unknown


extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// #[wasm_bindgen]
// pub fn say_hello_from_rust() {
//     log("Hello, from rust");
// }

#[wasm_bindgen]
pub struct DougsClient {

}

#[wasm_bindgen]
impl DougsClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {

        log("new() was hit");

        Self {

        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {

        log("update() was hit");

        Ok(())
    }

    pub fn render(&self) {

        log("render() was hit");

    }
}