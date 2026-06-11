// This file solves wasm issues with good code(generally complex syntax) -- so definitely implement in hot
// paths, for low-priority tasks, this is optional

use std::time::Instant;

use js_sys::Reflect;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::{console_log, now};

// Get access to a field of an object directly without conversion, without total copy.
#[wasm_bindgen]
pub fn config_engine_fast(raw_val: JsValue) {
    let start = now();
    for _ in 0..1_000_000 {
        // Get the raw js value(pointer) to the field 'username' of the Js object
        if let Ok(name_field) = Reflect::get(&raw_val, &JsValue::from_str("username")) {
            // Check if the field is some string
            if let Some(name) = name_field.as_string() {
                // console_log(&format!("Received connection setup request from {}", name));
            }
        }
    }
    let end = now();
    console_log(&format!("Time in ms for good engine: {}", (end - start)));
}

// Get access to a field of an object directly without conversion, without total copy.
#[wasm_bindgen]
pub fn config_engine_fast_single_conv(raw_val: JsValue) {
    let name_field =
        Reflect::get(&raw_val, &JsValue::from_str("username")).expect("The field doesn't exist");
    let name = name_field.as_string().expect("Username must be a string");
    let mut total_processed_bytes = 0;

    let start = now();
    for _ in 0..1_000_000 {
        total_processed_bytes += name.len();
    }
    let end = now();
    console_log(&format!(
        "Time in ms for good engine with single conversion: {}",
        (end - start)
    ));
}
