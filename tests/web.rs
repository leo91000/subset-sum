#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate wasm_subset_sum;
use subset_sum::get_subset_sum;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
fn expect_subset_sum_result(list: Vec<i32>, sum: i32) {
    if let Ok(Some(result)) = get_subset_sum(list, sum, None) {
        let result_sum: i32 = result.iter().sum();
        assert_eq!(result_sum, sum);
        return;
    }
    unreachable!();
}

#[cfg(test)]
fn expect_subset_sum_no_result(list: Vec<i32>, sum: i32) {
    if let Ok(None) = get_subset_sum(list, sum, None) {
        return;
    }
    unreachable!();
}

#[wasm_bindgen_test]
pub fn test_get_subset_with_match() {
    expect_subset_sum_result(vec![3, 5, 4], 0);
    expect_subset_sum_result(vec![3, 4], 7);
    expect_subset_sum_result(vec![3, 5, 4], 7);
    expect_subset_sum_result(vec![3, 5, 4], 9);
    expect_subset_sum_result(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 60);
    expect_subset_sum_result(vec![2, 3, 7, 4, 11, 5, 6, 8, 9, 1, 10], 60);
    expect_subset_sum_result(vec![-1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 60);
}

#[wasm_bindgen_test]
pub fn test_get_subset_without_match() {
    expect_subset_sum_no_result(vec![], 1);
    expect_subset_sum_no_result(vec![3, 6], 1);
    expect_subset_sum_no_result(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], -1);
    expect_subset_sum_no_result(vec![2, 3, 7, 4, 11, 5, 6, 8, 9, -1, 10], -2);
}
