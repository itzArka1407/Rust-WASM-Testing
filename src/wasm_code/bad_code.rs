// This file solves wasm issues with bad code(generally simple in terms of syntax) -- so avoid in
// hot paths

use std::time::Instant;

use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsValue, prelude::*};

use crate::{console_log, now};

#[derive(Serialize, Deserialize)]
pub struct UserConfig {
    pub username: String,
    pub connections: u32,
    pub use_tls: bool,
}

// This converts the whole js object into rust type -- has high overhead,
// but is robust and simple to use
#[wasm_bindgen]
pub fn config_engine_slowly(val: JsValue) {
    let start = now();
    for _ in 0..1_000_000 {
        let config: UserConfig = serde_wasm_bindgen::from_value(val.clone())
            .expect("Invalid object sent from javascript");
    }
    let end = now();
    console_log(&format!("Time in ms for bad engine: {}", (end - start)));
}
