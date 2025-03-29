//! Contains Duplicate
//! 
//! Given an integer array nums, return true if
//! any value appears at least twice in the array,
//!  and return false if every element is distinct.
//! 
//! 


use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::<i32>::new();
    set.extend(nums.iter());
    set.len() != nums.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {        
    }
}