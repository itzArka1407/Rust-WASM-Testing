use bitcode::{Buffer, Decode, Encode};
use gloo_net::http::Request;
use js_sys::Uint8Array;
use std::{cell::UnsafeCell, mem::MaybeUninit, sync::OnceLock};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen]
pub struct Client {
    server_resp: Vec<u8>, // The server response(bytes)
    buf: Buffer,          // The buffer to evaluate operations
}

#[wasm_bindgen]
#[derive(Encode)]
pub struct ClientRequest {
    name: String,   // Name of the client
    prompt: String, // Some random prompt
    id_hash: usize, // Cryptic Hash of the id of the client
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(expected_max_msg_size: usize) -> Self {
        Self {
            server_resp: Vec::with_capacity(expected_max_msg_size),
            buf: Buffer::new(),
        }
    }

    // Fetch something from the server
    #[wasm_bindgen]
    pub async fn fetch(&mut self, endpoint: &str, req: &ClientRequest) -> Result<JsValue, String> {
        let encoded = self.buf.encode(req);
        let resp = Request::post(endpoint) // Server endpoint
            .header("Content-Type", "application/octet-stream")
            .body(Uint8Array::from(encoded)) // The body is the encoded bytes
            .map_err(|e| e.to_string())?
            .send() // Send to server
            .await
            .map_err(|e| e.to_string())?; // Server response - bitcode format

        Err("None".to_owned())
    }
}
