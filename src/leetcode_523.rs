//! 523. Continuous Subarray Sum
//! 
//! Given an integer array nums and an integer k, return true if nums has a good subarray or false otherwise.
//! 
//! A good subarray is a subarray where:
//! its length is at least two, and
//! the sum of the elements of the subarray is a multiple of k.
//! 
//! Note that:
//! A subarray is a contiguous part of the array.
//! An integer x is a multiple of k if there exists an integer n such that x = n * k. 0 is always a multiple of k
//! 
//! 
//! 


mod Solution {


    // there is a clever solution using properties of modular arithmetic:
    // https://algo.monster/liteproblems/523


    /// Bruteforce approach
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        
        assert!(nums.len() > 1);

        let mut window = 2;

        while window < nums.len() {

            let mut offset = 0;
            while (offset + window ) < nums.len() {
                let mut sum = 0;
                for i in offset..offset+window {
                    sum += nums[i];
                }

                if sum == 0 || sum % k == 0 {
                    return true;
                }

                offset += 1;
            }

            window += 1;
        }

        false
    }
}


#[cfg(test)]
mod tests {
    use super::Solution::*;

    #[test]
    fn test_check_subarray_sum1() {    
        assert_eq!(check_subarray_sum(vec![23,2,4,6,7], 6), true);
        assert_eq!(check_subarray_sum(vec![23,2,6,4,7], 6), true);
        assert_eq!(check_subarray_sum(vec![23,2,6,4,7], 13), false);        
    }
}