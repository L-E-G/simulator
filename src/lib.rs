use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: String);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(format!("Hello {}", name));
}
