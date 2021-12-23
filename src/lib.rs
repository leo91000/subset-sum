mod subset_sum;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "getSubsetSum")]
pub fn get_subset_sum(
    list: Vec<i32>,
    sum: i32,
    timeout: Option<u32>,
) -> Result<Option<Vec<i32>>, JsValue> {
    let results = subset_sum::get_subset_sum(list, sum, timeout.map(|timeout| timeout as u128))?;
    Ok(results)
}
