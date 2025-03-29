//! Product of Array Except Self
//! Given an integer array nums, return an array answer such that answer[i] 
//!     is equal to the product of all the elements of nums except nums[i].
//! The product of any prefix or suffix of nums is guaranteed to fit in a 
//!     32-bit integer.
//! You must write an algorithm that runs in O(n) time
//!     and without using the division operation.
//! 
//! Example 1:
//! Input: nums = [3, 2, 3, 4 ]
//! ///            1  3  6  18
//!                24 36 24 18
//! Output: [24,12,8,6]
//! 
//! Example 2:
//! Input: nums = [-1,1,0,-3,3]
//!                  
//! Output: [0,0,9,0,0]

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    
    let mut res = Vec::<i32>::new();

    let mut left_product = 1;
    res.push(left_product);

    for jj in 1..nums.len() {  
        left_product = left_product * nums[jj-1];
        res.push(left_product);
    }

    let mut right_product = 1;
    for jj in (0..(nums.len() - 1)).rev() {  
        right_product = right_product * nums[jj+1];
        res[jj] = res[jj] * right_product;
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {        
        assert!(product_except_self(vec![3, 2, 3, 4 ]) == vec![24, 36, 24, 18]);
        assert!(product_except_self(vec![-1,1,0,-3,3]) == vec![0,0,9,0,0]);
    }
}