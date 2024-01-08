use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn remove_duplicates_old(nums: &mut Vec<i32>) -> i32 {
        let nums_clone = nums.clone();
        let mut idxs_to_remove = vec![];
        for (idx, num) in nums_clone.iter().enumerate() {
            let first_idx = nums_clone.iter().position(|&x| x == *num).unwrap();
            // let last_idx = nums_clone.iter().rposition(|&x| x == *num).unwrap();
            if first_idx != idx {
                //nums.remove(last_idx);
                println!("Should remove {}", idx);
                //if !idxs_to_remove.contains(&last_idx) {
                idxs_to_remove.push(idx);
                //}
            }
        }

        // reverse idxs_to_remove
        for idx in idxs_to_remove.iter().rev() {
            nums.remove(*idx);
        }

        println!("{:?}", nums);

        return nums.len() as i32;
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let nums_clone = nums.clone();
        let mut idxs_to_remove = vec![];
        let mut first_idxs = HashMap::new();
        for (idx, num) in nums_clone.iter().enumerate() {
            if !first_idxs.contains_key(num) {
                first_idxs.insert(num, idx);
            } else {
                println!("Should remove {}", idx);
                idxs_to_remove.push(idx);
            }
        }

        // reverse idxs_to_remove
        for idx in idxs_to_remove.iter().rev() {
            nums.remove(*idx);
        }

        println!("{:?}", nums);

        return nums.len() as i32;
    }
}
