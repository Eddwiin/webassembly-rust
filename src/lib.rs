// src/lib.rs
use wasm_bindgen::prelude::*;

mod streaming;
pub use streaming::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn add_nums(a: i32, b: i32) -> String  {
    let val = a + b;
    format!("Hello world {}", val)
}