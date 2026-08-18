use bitcode::{Buffer, Decode, Encode};
use gloo_net::http::{Request, Response};
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use std::{cell::UnsafeCell, mem::MaybeUninit, sync::OnceLock};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
/// Internal representation of the client for server-client communication
pub struct Client {
    server_resp: Vec<u8>, // The server response(bytes)
}

#[derive(Encode)]
#[wasm_bindgen]
pub struct ClientRequest {
    name: String,   // Name of the client
    prompt: String, // Some random prompt
    id_hash: usize, // Cryptic Hash of the id of the client
}

/// Response from the server
#[derive(Decode, Serialize)]
#[wasm_bindgen]
pub struct ServerResp {
    authentication_key: Option<u128>, // may or may not be authenticated(with the key)
    target_name: String,
    target_age: u8,
    target_nationality: String,
}

impl Client {
    pub fn new(expected_max_msg_size: usize) -> Self {
        Self {
            server_resp: Vec::with_capacity(expected_max_msg_size),
        }
    }

    // Fetch something from the server
    pub async fn fetch(&self, endpoint: &str, data: &[u8]) -> Result<Vec<u8>, String> {
        let resp = Request::post(endpoint) // Server endpoint
            .header("Content-Type", "application/octet-stream")
            .body(Uint8Array::from(data)) // The body is the encoded bytes
            .map_err(|e| e.to_string())?
            .send() // Send to server
            .await
            .map_err(|e| e.to_string())?; // Server response - bitcode format

        let bytes = resp.binary().await.map_err(|e| e.to_string())?;
        Ok(bytes)
    }
}

/// Wrapper over internal client to communicate with JS
#[wasm_bindgen]
pub struct WasmClient(Client, Buffer);

#[wasm_bindgen]
impl WasmClient {
    #[wasm_bindgen(constructor)]
    pub fn new(expected_max_msg_size: usize) -> Self {
        Self(Client::new(expected_max_msg_size), Buffer::new())
    }

    /// Send a request to server
    #[wasm_bindgen]
    pub async fn send_req(
        &mut self,
        endpoint: &str,
        req: ClientRequest,
    ) -> Result<JsValue, String> {
        let data = self.1.encode(&req);

        // Send the data to the client
        let res = self.0.fetch(endpoint, data).await?;
        let final_res: ServerResp = bitcode::decode(&res).map_err(|e| e.to_string())?;

        Ok(to_value(&final_res).map_err(|e| e.to_string())?)
    }
}
