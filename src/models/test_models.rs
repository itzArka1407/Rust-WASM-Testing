use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct TesterStruct {
    pub name: String,
    pub id: Uuid,
    pub age: usize,
}

#[derive(Encode, Decode)]
pub struct TesterStruct2 {
    pub name: String,
    pub id: Uuid,
    pub age: usize,
}
