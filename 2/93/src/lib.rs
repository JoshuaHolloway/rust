// > npm run dev
// > npm run build

// Don't need to repeat again:
// > rustup target add wasm32-unknown-unknown

// > cargo build --target wasm32-unknown-unknown

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

mod gl_setup;

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
    gl: WebGlRenderingContext,
}

#[wasm_bindgen]
impl DougsClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        Self {
            // Store graphics layer when we initialize DougsClient
            gl: gl,
            // _program_color_2d: programs::Color2D::new(&gl),
            // _program_color_2d_gradient: programs::Color2DGradient::new(&gl),
            // program_graph_3d: programs::Graph3D::new(&gl),
            // gl: gl,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        log("update() was hit");

        Ok(())
    }

    pub fn render(&self) {
        log("render() was hit");
        // clear screen
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }
}

// > npm run dev
// > cargo build
