pub use crate::error::SubsetSumError;
use fn_memo::recur_fn::RecurFn;
use fn_memo::{unsync, FnMemo};
#[cfg(feature = "wasm-js")]
use instant::Instant;
use num_traits::Num;
use std::hash::Hash;
use std::iter::Sum;
#[cfg(not(feature = "wasm-js"))]
use std::time::Instant;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct SubsetSumArg<'a, N: Num + Copy> {
    integer_list: &'a [N],
    sum: N,
}

struct SubsetSum {
    now: Instant,
    timeout_in_ms: Option<u128>,
}

pub type SubsetSumResult<N> = Result<Option<Vec<N>>, SubsetSumError>;

impl<'a, N> RecurFn<SubsetSumArg<'a, N>, SubsetSumResult<N>> for SubsetSum
where
    N: Num + Copy + Hash + Sum + Eq + Ord,
{
    #[inline]
    fn body(
        &self,
        subset_sum: impl Fn(SubsetSumArg<'a, N>) -> SubsetSumResult<N>,
        arg: SubsetSumArg<'a, N>,
    ) -> SubsetSumResult<N> {
        if let Some(timeout) = self.timeout_in_ms {
            if self.now.elapsed().as_millis() >= timeout {
                return Err(SubsetSumError::ExecutionTimeout);
            }
        }

        if arg.sum.is_zero() {
            return Ok(Some(vec![]));
        }

        if arg.integer_list.is_empty() {
            return Ok(None);
        }

        if arg.integer_list.iter().copied().sum::<N>() == arg.sum {
            return Ok(Some(arg.integer_list.to_vec()));
        }

        if arg.integer_list.contains(&arg.sum) {
            return Ok(Some(vec![arg.sum]));
        }

        for (index, &current) in arg.integer_list.iter().enumerate() {
            let subset = &arg.integer_list[index + 1..];

            if let Some(mut result) = subset_sum(SubsetSumArg {
                integer_list: subset,
                sum: arg.sum - current,
            })? {
                result.push(current);
                return Ok(Some(result));
            }
        }

        Ok(None)
    }
}

pub fn get_subset_sum<N>(
    mut list: Vec<N>,
    sum: N,
    timeout_in_ms: Option<u128>,
) -> SubsetSumResult<N>
where
    N: Num + Copy + Hash + Eq + Ord + Sum,
{
    let subset_sum = unsync::memoize(SubsetSum {
        now: Instant::now(),
        timeout_in_ms,
    });

    list.sort_unstable();

    subset_sum.call(SubsetSumArg {
        integer_list: &list,
        sum,
    })
}

#[cfg(test)]
mod tests {
    use crate::get_subset_sum;

    #[test]
    fn it_should_return_empty_vec_if_sum_is_zero() {
        assert_eq!(
            get_subset_sum(vec![3, 4], 0, None).unwrap().unwrap(),
            vec![]
        );
    }

    fn expect_subset_sum_result(list: Vec<i32>, sum: i32) {
        if let Ok(Some(result)) = get_subset_sum(list, sum, None) {
            let result_sum: i32 = result.iter().sum();
            assert_eq!(result_sum, sum);
            return;
        }
        unreachable!();
    }

    #[test]
    fn it_should_return_a_vec_with_a_sum_equals_to_the_given_integer() {
        expect_subset_sum_result(vec![3, 5, 4], 0);
        expect_subset_sum_result(vec![3, 4], 7);
        expect_subset_sum_result(vec![3, 5, 4], 7);
        expect_subset_sum_result(vec![3, 5, 4], 9);
        expect_subset_sum_result(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 60);
        expect_subset_sum_result(vec![2, 3, 7, 4, 11, 5, 6, 8, 9, 1, 10], 60);
        expect_subset_sum_result(vec![3, 1, -1, 3], 6);
        expect_subset_sum_result(vec![-1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 60);
    }

    fn expect_subset_sum_no_result(list: Vec<i32>, sum: i32) {
        if let Ok(None) = get_subset_sum(list, sum, None) {
            return;
        }
        unreachable!();
    }

    #[test]
    fn it_should_return_no_match_error_when_there_is_no_sum() {
        expect_subset_sum_no_result(vec![], 1);
        expect_subset_sum_no_result(vec![3, 6], 1);
        expect_subset_sum_no_result(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], -1);
        expect_subset_sum_no_result(vec![2, 3, 7, 4, 11, 5, 6, 8, 9, -1, 10], -2);
    }
}
