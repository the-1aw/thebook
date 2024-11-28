use std::collections::HashMap;

fn get_frequency_map(nums: &[usize]) -> HashMap<usize, usize> {
    let mut frequency_map: HashMap<usize, usize> = HashMap::new();
    for num in nums {
        frequency_map
            .entry(*num)
            .and_modify(|frequency| *frequency += 1)
            .or_insert(1);
    }
    frequency_map
}
fn get_highest_frequency(nums: &[usize]) -> usize {
    let frequency_map = get_frequency_map(nums);
    let mut highest_frequency = 0;
    let mut most_frequent = 0;
    for (key, value) in frequency_map {
        if value > highest_frequency {
            most_frequent = key;
            highest_frequency = value;
        }
    }
    most_frequent
}
// Given a list of integers, return a vector containing the median and the mode
pub fn challenge_integer_list(nums: &[usize]) -> Vec<usize> {
    let mut nums = nums.to_vec();
    nums.sort();
    let median_idx = if nums.len() % 2 == 0 {
        (nums.len() / 2 + (nums.len() / 2 + 1)) / 2
    } else {
        (nums.len() + 1) / 2
    };
    let mut result = Vec::with_capacity(2);
    result.push(nums[median_idx]);
    result.push(get_highest_frequency(&nums));
    result
}
