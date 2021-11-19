#[derive(strum_macros::Display)]
pub enum SubsetSumError {
    NoMatch
}

pub fn find_first_subset_sum(list: Vec<i32>, sum: i32) -> Result<Vec<i32>, SubsetSumError> {
    match list.iter().find(|&&v| v == sum) {
        Some(&value) => return Ok(vec![value]),
        None => {
            let total: i32 = list.iter().sum();
            if total == sum {
                return Ok(list);
            }

            for (index, &current) in list.iter().enumerate() {
                let mut subset = list.clone();
                subset.swap_remove(index);
                match find_first_subset_sum(subset, sum - current) {
                    Ok(mut result) => {
                        result.push(current);
                        return Ok(result)
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
    }

    Err(SubsetSumError::NoMatch)
}
