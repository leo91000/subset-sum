mod subset_sum;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_subset_sum(list: Vec<i32>, sum: i32) -> Result<Vec<i32>, JsValue> {
    match subset_sum::get_subset_sum(list, sum) {
        Ok(result) => Ok(result),
        Err(e) => Err(JsValue::from(e.to_string())),
    }
}
