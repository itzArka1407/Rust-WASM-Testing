mod models;
mod tests;
use wasm_bindgen::prelude::*;

// Importing a JS fn to rust
#[wasm_bindgen]
extern "C" {
    // Native window.alert function in JS
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let message = format!("Hello {} from rust", name);
    alert(&message);
}
