mod utils;
mod subset_sum;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn find_first_subset_sum(list: Vec<i32>, sum: i32) -> Result<Vec<i32>, JsValue> {
    match subset_sum::find_first_subset_sum(list, sum) {
        Ok(result) => Ok(result),
        Err(e) => Err(JsValue::from(e.to_string()))
    }
}
