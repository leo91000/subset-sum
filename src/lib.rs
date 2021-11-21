mod subset_sum;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_subset_sum(
    list: Vec<i32>,
    sum: i32,
    timeout: Option<u32>,
) -> Result<Option<Vec<i32>>, JsValue> {
    let u128_timeout = timeout.map(|timeout| timeout as u128);

    match subset_sum::get_subset_sum(list, sum, u128_timeout) {
        Ok(result) => Ok(result),
        Err(e) => Err(JsValue::from(e.to_string())),
    }
}
