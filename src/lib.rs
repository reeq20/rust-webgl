use wasm_bindgen::prelude::*;
// use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    pub fn start();
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn return_str() -> String {
    let s1 = String::from("abc");
    s1
}