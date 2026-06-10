// SAFETY: This is for dev purposes only
#![allow(unused)]

mod models;
mod tests;
mod wasm_code;
use wasm_bindgen::prelude::*;

// Importing a JS fn to rust
#[wasm_bindgen]
extern "C" {
    // By default for a fn with fn_name as name, if no namespace/name is provided, the fn becomes window.<fn_name>

    // Native window.alert function in JS
    #[wasm_bindgen(js_namespace = window, js_name = alert)]
    fn window_alert(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str); // console.log in js
}

// Accept a js object(the JsValue)
#[wasm_bindgen]
pub fn accept_js_obj(val: JsValue) {
    if val.is_object() {
        window_alert("Obj sent to fn");
        console_log("Object received");
    }
}

// Work on a native js array(e.g: Uint8Array) -- can be directly referenced without copy
#[wasm_bindgen]
pub fn calc_sum(data: &[u8]) -> u32 {
    let mut sum = 0;
    for val in data {
        sum += *val as u32;
    }
    sum
}
