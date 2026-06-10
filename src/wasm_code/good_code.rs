// This file solves wasm issues with good code(generally complex syntax) -- so definitely implement in hot
// paths, for low-priority tasks, this is optional

use js_sys::Reflect;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::console_log;

// Get access to a field of an object directly without conversion, without total copy.
#[wasm_bindgen]
pub fn config_engine_fast(raw_val: JsValue) {
    // Get the raw js value(pointer) to the field 'username' of the Js object
    if let Ok(name_field) = Reflect::get(&raw_val, &JsValue::from_str("username")) {
        // Check if the field is some string
        if let Some(name) = name_field.as_string() {
            console_log(&format!("Received connection setup request from {}", name));
        }
    }
}
