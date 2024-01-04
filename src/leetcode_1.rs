//! Leetcode 1: Two Sum
//! Given an array of integers nums and an integer target,
//!  return indices of the two numbers such that they add up to target.

//!You may assume that each input would 
//! have exactly one solution, and you may not use the same element twice.
//! You can return the answer in any order.

use std::collections::HashMap;

// O(n^2) solution
pub fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();

    if len < 2 {
        return Vec::new();
    }

    if len == 2 {
        return vec![0, 1];
    }

    for i in 0..len {
        for j in (i+1)..len {
            if (nums[i] + nums[j]) == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    Vec::new()
}

pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();

    if len < 2 {
        return Vec::new();
    }

    if len == 2 {
        return vec![0, 1];
    }

    let map: HashMap::<i32, usize> = nums.iter().enumerate().map(|(i, x)| {
        (*x, i)
    }).collect();
    
    for i in 0..len {
        let x = target - nums[i];
        if let Some(y) = map.get(&x) {
            if *y != i {
                return vec![i as i32, *y as i32];
            }
        }
    }

    Vec::new()
}


pub fn two_sum_3(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();

    if len < 2 {
        return Vec::new();
    }

    if len == 2 {
        return vec![0, 1];
    }

    let mut map: HashMap::<i32, usize> = HashMap::new();
    
    for i in 0..len {
        let num = nums[i];
        let x = target - num;
        if let Some(y) = map.get(&x) {
            return vec![i as i32, *y as i32];  
        } else {
            map.insert(num, i);
        }
    }

    Vec::new()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let result = two_sum_2(vec![3,2,4], 6);
        assert_eq!(result, vec![1, 2]);
    }
}