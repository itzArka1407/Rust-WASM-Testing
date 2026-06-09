use std::time::Instant;

use bitcode::Buffer;
use uuid::Uuid;

use crate::models::test_models::{TesterStruct, TesterStruct2};

#[test]
fn bitcode_outputs() {
    let test_inst = TesterStruct {
        name: "Jack".to_string(),
        id: Uuid::now_v7(),
        age: 40,
    };

    let test_inst2 = TesterStruct2 {
        name: "Jack".to_string(),
        id: Uuid::now_v7(),
        age: 40,
    };

    let iterations = 1_000_000;

    let inst = Instant::now();
    for _ in 0..iterations {
        _ = bitcode::serialize(&test_inst).unwrap();
    }
    println!(
        "Average time taken for serialize: {}",
        inst.elapsed().as_nanos() / iterations
    );

    let mut buf = Buffer::new();
    let inst = Instant::now();
    for _ in 0..iterations {
        _ = buf.encode(&test_inst2);
    }
    println!(
        "Average time taken for encode: {}",
        inst.elapsed().as_nanos() / iterations
    );

    // RESULTS:
    // bitcode::serialize --> avg: 950 nano seconds
    // bitcode::encode --> avg: 460 nano seconds
    // bitcode::Buffer::encode --> avg: 108 nano seconds
    // CPU - ryzen 7 7735HS, 16gb DDR5

    let enc1 = bitcode::serialize(&test_inst).unwrap();
    let enc2 = bitcode::encode(&test_inst2);
    let enc3 = buf.encode(&test_inst2);

    println!("{enc1:?}");
    println!("{enc2:?}");
    println!("{enc3:?}");
}

