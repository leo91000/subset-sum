use fn_memo::{recur_fn::recur_fn, unsync, FnMemo};

#[derive(strum_macros::Display, Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubsetSumError {
    NoMatch,
    ExecutionTimeout,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct SubsetSumArg {
    integer_list: Vec<i32>,
    sum: i32,
}

pub fn get_subset_sum(list: Vec<i32>, sum: i32) -> Result<Vec<i32>, SubsetSumError> {
    let subset_sum = unsync::memoize(recur_fn(
        |subset_sum, arg: SubsetSumArg| -> Result<Vec<i32>, SubsetSumError> {
            if arg.sum == 0 {
                return Ok(vec![]);
            }

            if arg.integer_list.is_empty() {
                return Err(SubsetSumError::NoMatch);
            }

            for (index, &current) in arg.integer_list.iter().enumerate() {
                let mut subset = arg.integer_list.clone();
                subset.remove(index);
                match subset_sum(SubsetSumArg {
                    integer_list: subset,
                    sum: arg.sum - current,
                }) {
                    Ok(mut result) => {
                        if index == 1 {
                            result.push(current);
                        } else {
                            result.insert(index, current);
                        }
                        return Ok(result);
                    }
                    Err(e) => {
                        if e == SubsetSumError::ExecutionTimeout {
                            return Err(e);
                        }
                    }
                }
            }

            Err(SubsetSumError::NoMatch)
        },
    ));

    let mut ordered_list = list;
    ordered_list.sort_unstable();

    subset_sum.call(SubsetSumArg {
        integer_list: ordered_list,
        sum,
    })
}

#[cfg(test)]
mod tests {
    use crate::subset_sum::{get_subset_sum, SubsetSumError};

    #[test]
    fn it_should_return_empty_vec_if_sum_is_zero() {
        assert_eq!(get_subset_sum(vec![3, 4], 0).unwrap(), vec![]);
    }

    fn expect_subset_sum_ok(list: Vec<i32>, sum: i32) {
        match get_subset_sum(list, sum) {
            Ok(result) => {
                let result_sum: i32 = result.iter().sum();
                assert_eq!(result_sum, sum);
            }
            _ => {
                unreachable!();
            }
        }
    }

    #[test]
    fn it_should_return_a_vec_with_a_sum_equals_to_the_given_integer() {
        expect_subset_sum_ok(vec![3, 5, 4], 0);
        expect_subset_sum_ok(vec![3, 4], 7);
        expect_subset_sum_ok(vec![3, 5, 4], 7);
        expect_subset_sum_ok(vec![3, 5, 4], 9);
        expect_subset_sum_ok(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 60);
        expect_subset_sum_ok(vec![2, 3, 7, 4, 11, 5, 6, 8, 9, 1, 10], 60);
        expect_subset_sum_ok(vec![-1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 60);
    }

    fn expect_subset_sum_err(list: Vec<i32>, sum: i32) {
        match get_subset_sum(list, sum) {
            Err(result) => {
                assert_eq!(result, SubsetSumError::NoMatch);
            }
            _ => {
                unreachable!();
            }
        }
    }

    #[test]
    fn it_should_return_no_match_error_when_there_is_no_sum() {
        expect_subset_sum_err(vec![], 1);
        expect_subset_sum_err(vec![3, 6], 1);
        expect_subset_sum_err(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], -1);
        expect_subset_sum_err(vec![2, 3, 7, 4, 11, 5, 6, 8, 9, -1, 10], -2);
    }
}
