use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn sum_of_n_ints(n: i32) -> i32 {
    n * (n + 1) / 2
}