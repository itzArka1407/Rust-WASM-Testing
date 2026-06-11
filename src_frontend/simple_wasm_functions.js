// 1. Updated import to match your actual build output file name
import { accept_js_obj, calc_sum, config_engine_slowly, config_engine_fast, config_engine_fast_single_conv } from '../pkg/wasm_bitcode_demo.js';

function send_obj() {
    const obj = { name: "Test", id: 62 };
    accept_js_obj(obj);
}

function get_sum_of_arr() {
    const test_arr = new Uint8Array([10, 20, 30, 40, 50]);
    let sum = calc_sum(test_arr);
    console.log("Sum of array: ", sum);
}

function evaluate_connection_slow() {
    const userConfig = { username: "Jack", connections: 10, use_tls: false };
    config_engine_slowly(userConfig);
}

function evaluate_connection_fast() {
    const userConfig = { username: "JackTheReaper", connections: 100 };
    config_engine_fast(userConfig);
}

function evaluate_single_conv() {
    const userConfig = { username: "JackTheReaper1407" };
    config_engine_fast_single_conv(userConfig);
}

export default {
    send_obj,
    get_sum_of_arr,
    evaluate_connection_slow,
    evaluate_connection_fast,
    evaluate_single_conv
};
