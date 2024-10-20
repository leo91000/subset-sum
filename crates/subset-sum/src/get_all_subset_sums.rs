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
pub struct SubsetSumAllArg<'a, N: Num + Copy> {
    integer_list: &'a [N],
    sum: N,
}

struct SubsetSumAll {
    now: Instant,
    timeout_in_ms: Option<u128>,
}

pub type SubsetSumAllResult<N> = Result<Vec<Vec<N>>, SubsetSumError>;

impl<'a, N> RecurFn<SubsetSumAllArg<'a, N>, SubsetSumAllResult<N>> for SubsetSumAll
where
    N: Num + Copy + Hash + Eq + Ord,
{
    fn body(
        &self,
        subset_sum_all: impl Fn(SubsetSumAllArg<'a, N>) -> SubsetSumAllResult<N>,
        arg: SubsetSumAllArg<'a, N>,
    ) -> SubsetSumAllResult<N> {
        if let Some(timeout) = self.timeout_in_ms {
            if self.now.elapsed().as_millis() >= timeout {
                return Err(SubsetSumError::ExecutionTimeout);
            }
        }

        if arg.sum.is_zero() {
            return Ok(vec![vec![]]);
        }

        if arg.integer_list.is_empty() {
            return Ok(vec![]);
        }

        let mut subsets = Vec::new();

        let current = arg.integer_list[0];
        let rest = &arg.integer_list[1..];

        // Include current
        if arg.sum >= current {
            let include_current = subset_sum_all(SubsetSumAllArg {
                integer_list: rest,
                sum: arg.sum - current,
            })?;
            for mut subset in include_current {
                subset.push(current);
                subsets.push(subset);
            }
        }

        // Exclude current
        let exclude_current = subset_sum_all(SubsetSumAllArg {
            integer_list: rest,
            sum: arg.sum,
        })?;
        subsets.extend(exclude_current);

        Ok(subsets)
    }
}

pub fn get_all_subset_sums<N>(
    mut list: Vec<N>,
    sum: N,
    timeout_in_ms: Option<u128>,
) -> SubsetSumAllResult<N>
where
    N: Num + Copy + Hash + Eq + Ord + Sum,
{
    let subset_sum_all = unsync::memoize(SubsetSumAll {
        now: Instant::now(),
        timeout_in_ms,
    });

    // Sort the list to handle duplicates appropriately
    list.sort_unstable();

    subset_sum_all.call(SubsetSumAllArg {
        integer_list: &list,
        sum,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn collect_subsets<N>(result: Vec<Vec<N>>) -> HashSet<Vec<N>>
    where
        N: Num + Copy + Ord + Hash,
    {
        result
            .into_iter()
            .map(|mut subset| {
                subset.sort_unstable();
                subset
            })
            .collect()
    }

    #[test]
    fn test_get_all_subset_sums_basic() {
        let list = vec![1, 3, 3, 3, 7];
        let sum = 10;
        let result = get_all_subset_sums(list, sum, None).unwrap();

        let subsets = collect_subsets(result);

        let expected_subsets: HashSet<Vec<i32>> =
            vec![vec![1, 3, 3, 3], vec![3, 7]].into_iter().collect();

        assert_eq!(subsets, expected_subsets);
    }

    #[test]
    fn test_get_all_subset_sums_empty_list() {
        let list: Vec<i32> = vec![];
        let sum = 0;
        let result = get_all_subset_sums(list, sum, None).unwrap();

        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn test_get_all_subset_sums_zero_sum() {
        let list = vec![1, 2, 3];
        let sum = 0;
        let result = get_all_subset_sums(list, sum, None).unwrap();

        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn test_get_all_subset_sums_negative_numbers() {
        let list = vec![3, -1, -2, 5];
        let sum = 2;
        let result = get_all_subset_sums(list, sum, None).unwrap();

        let subsets = collect_subsets(result);

        let expected_subsets: HashSet<Vec<i32>> =
            vec![vec![-2, -1, 5], vec![-1, 3]].into_iter().collect();

        assert_eq!(subsets, expected_subsets);
    }

    #[test]
    fn test_get_all_subset_sums_with_duplicates() {
        let list = vec![2, 2, 2, 2];
        let sum = 4;
        let result = get_all_subset_sums(list, sum, None).unwrap();

        let subsets = collect_subsets(result);

        let expected_subsets: HashSet<Vec<i32>> = vec![vec![2, 2]].into_iter().collect();

        assert_eq!(subsets, expected_subsets);
    }

    #[test]
    fn test_get_all_subset_sums_large_sum() {
        let list = vec![1, 2, 3, 4, 5];
        let sum = 15;
        let result = get_all_subset_sums(list, sum, None).unwrap();

        let subsets = collect_subsets(result);

        let expected_subsets: HashSet<Vec<i32>> = vec![vec![1, 2, 3, 4, 5]].into_iter().collect();

        assert_eq!(subsets, expected_subsets);
    }

    #[test]
    fn test_get_all_subset_sums_no_result() {
        let list = vec![1, 2, 3];
        let sum = 7;
        let result = get_all_subset_sums(list, sum, None).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn test_get_all_subset_sums_negative_sum() {
        let list = vec![1, -2, 3, -4, 5];
        let sum = -1;
        let result = get_all_subset_sums(list, sum, None).unwrap();

        let subsets = collect_subsets(result);

        let expected_subsets: HashSet<Vec<i32>> = vec![
            vec![-4, 3],     // -4 + 3 = -1
            vec![-2, 1],     // -2 + 1 = -1
            vec![-4, -2, 5], // -4 + (-2) + 5 = -1
        ]
        .into_iter()
        .collect();

        assert_eq!(subsets, expected_subsets);
    }

    #[test]
    fn test_get_all_subset_sums_with_zeros() {
        let list = vec![0, 0, 0, 1];
        let sum = 1;
        let result = get_all_subset_sums(list, sum, None).unwrap();

        let subsets = collect_subsets(result);

        let expected_subsets: HashSet<Vec<i32>> =
            vec![vec![1], vec![0, 1], vec![0, 0, 1], vec![0, 0, 0, 1]]
                .into_iter()
                .collect();

        assert_eq!(subsets, expected_subsets);
    }

    #[test]
    fn test_get_all_subset_sums_timeout() {
        let list = vec![1; 25]; // A list with 25 ones
        let sum = 20;
        let timeout_ms = Some(1); // Set a very short timeout

        let result = get_all_subset_sums(list, sum, timeout_ms);

        assert!(result.is_err());
        if let Err(SubsetSumError::ExecutionTimeout) = result {
            // Expected
        } else {
            panic!("Expected ExecutionTimeout error");
        }
    }
}
