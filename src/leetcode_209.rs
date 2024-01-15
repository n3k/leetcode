//! Leetcode 209: Minimum Size Subarray Sum
//!
//! Given an array of positive integers nums 
//! and a positive integer target, return the minimal 
//! length of a subarray whose sum is greater than or 
//! equal to target. If there is no such subarray, return 0 instead.

mod Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut res = i32::MAX;

        let len = nums.len();
        if len == 0 {
            return 0;
        }

        if len < 2 {
            if nums[0] >= target {
                return 1;
            } else {
                return 0;
            }
        }

        let mut l = 0usize;
        let mut r = 1usize;
        let mut sum = nums[0];

        loop {
            if r == len {
                if sum < target { 
                    if res == i32::MAX {
                        return 0;
                    } 

                    return res;
                }
                //println!("   r:[{}] l:[{}]", r - 1, l);

                while sum >= target {
                    sum -= nums[l];
                    l += 1;
                }

                l -= 1;
                return core::cmp::min(res, (r - l) as i32); 
            }

            
           
            while sum >= target {
                //println!("rx:[{}] lx:[{}] --> sum={}  len={}", r , l, sum, r - l);
                res = core::cmp::min(res, (r - l) as i32);
                sum -= nums[l];
                l += 1;
            }

            let num = nums[r];
            if num >= target {
                return 1;
            }

            sum += num;
            r += 1;
        }    
    }           
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        assert_eq!(Solution::min_sub_array_len(7 , vec![2,3,1,2,4,3]), 2);
        assert_eq!(Solution::min_sub_array_len(4 , vec![1,4,4]), 1);
        assert_eq!(Solution::min_sub_array_len(11, vec![1,1,1,1,1,1,1,1]), 0);
        // [1, 2, 3, 4, 5]
        assert_eq!(Solution::min_sub_array_len(11, vec![1,2,3,4,5]), 3);

        assert_eq!(Solution::min_sub_array_len(
            213, vec![12,  28,83,4,25,26,25,2,25,  25,25,12]),
            8);
    }
}