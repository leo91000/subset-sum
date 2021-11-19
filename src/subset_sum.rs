#[derive(strum_macros::Display)]
pub enum SubsetSumError {
    NoMatch
}

pub fn find_first_subset_sum(list: Vec<i32>, sum: i32) -> Result<Vec<i32>, SubsetSumError> {
    {
        let mut current_sum = 0;
        for (index, &current) in list.iter().enumerate() {
            if current == sum {
                return Ok(vec![current])
            }
            current_sum += current;
            if sum == current_sum {
                let mut result = list.clone();
                result.truncate(index + 1);
                return Ok(result)
            }
        }
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
    Err(SubsetSumError::NoMatch)
}