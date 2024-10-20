#[macro_use]
extern crate napi_derive;

use napi::JsError;

#[napi]
pub fn get_subset_sum(
    list: Vec<i32>,
    sum: i32,
    timeout: Option<u32>,
) -> Result<Option<Vec<i32>>, JsError> {
    let results = subset_sum::get_subset_sum(list, sum, timeout.map(|timeout| timeout as u128))?;
    Ok(results)
}

#[napi]
pub fn get_all_subset_sums(
    list: Vec<i32>,
    sum: i32,
    timeout: Option<u32>,
) -> Result<Vec<Vec<i32>>, JsError> {
    let results =
        subset_sum::get_all_subset_sums(list, sum, timeout.map(|timeout| timeout as u128))?;
    Ok(results)
}
