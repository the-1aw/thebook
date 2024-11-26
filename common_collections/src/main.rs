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
fn challenge_integer_list(nums: &[usize]) -> Vec<usize> {
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

// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay)
fn challenge_pig_lating() {}

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company
// for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
fn challenge_employee_list() {}

fn main() {
    let integer_list_odd = [1, 9, 12, 43, 9, 3, 32];
    let integer_list_even = [8, 9, 43, 42, 9, 132];
    assert_eq!(vec![12, 9], challenge_integer_list(&integer_list_odd));
    assert_eq!(vec![42, 9], challenge_integer_list(&integer_list_even));
    challenge_pig_lating();
    challenge_employee_list()
}
