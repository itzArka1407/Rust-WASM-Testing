// To implement a data pipeline in wasm using efficient code structure

use wasm_bindgen::prelude::wasm_bindgen;

use crate::console_log;

#[wasm_bindgen]
pub struct DataPipeline {
    // Fixed, pre allocated block to reduce allocator pressure and memory churn
    data: Vec<u8>,
}

#[wasm_bindgen]
impl DataPipeline {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
        }
    }

    pub fn get_data_ptr(&self) -> *const u8 {
        &self.data[0]
    }

    pub fn process(&mut self, bytes_written: usize) {
        console_log(&format!(
            "Len: {}, Cap: {}, Requested: {}",
            self.data.len(),
            self.data.capacity(),
            bytes_written,
        ));

        if bytes_written > self.data.len() {
            console_log("Out of bounds, too much data requested");
            return;
        }

        let target_bytes = &mut self.data[..bytes_written];
        for byte in target_bytes {
            // todo!("Perform operation on the bytes itself");
        }
    }
}
