use serde_wasm_bindgen::to_value;
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

#[wasm_bindgen(js_name = "getAllSubsetSums")]
pub fn get_all_subset_sum(
    list: Vec<i32>,
    sum: i32,
    timeout: Option<u32>,
) -> Result<JsValue, JsValue> {
    let results =
        subset_sum::get_all_subset_sums(list, sum, timeout.map(|timeout| timeout as u128))
            .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let js_value = to_value(&results).map_err(|err| JsValue::from_str(&err.to_string()))?;
    Ok(js_value)
}
