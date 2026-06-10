// This file solves wasm issues with bad code(generally simple in terms of syntax) -- so avoid in
// hot paths

use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsValue, prelude::*};

use crate::console_log;

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
    let config: UserConfig =
        serde_wasm_bindgen::from_value(val).expect("Invalid object sent from javascript");

    console_log(&format!(
        "Configured {} with {}",
        config.username, config.connections
    ));
}
