use std::collections::HashMap;
use std::convert::TryInto;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut data = HashMap::new();
        for (num_index, &num) in nums.iter().enumerate() {
            let idx = num_index.try_into().unwrap();
            if let Some(eventually_found_option) = data.get(&num) {
                return vec![*eventually_found_option, idx];
            }
            data.insert(target - num, idx);
        }

        vec![]
    }
}
