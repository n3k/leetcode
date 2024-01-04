//! Leetcode 53: Maximum Subarray
//! 
//! Given an integer array nums, find the contiguous
//! subarray (containing at least one number) which has
//! the largest sum and return its sum

// O(n^2) Solution
fn max_subarray1(array: Vec<i32>) -> i32 {

    let len = array.len();

    if len == 0 {
        return 0;
    }

    if len < 2 {
        return array[0];
    }

    let mut sum;
    let mut max_sum = i32::MIN;
    
    for i in 0..len {
        sum = 0;
        for j in i..len {
            sum = sum + array[j];
            max_sum = core::cmp::max(max_sum, sum);
        }
    }

    max_sum
}


/// Linear solution
fn max_subarray2(array: Vec<i32>) -> i32 {

    let len = array.len();
    let mut sum = 0;
    let mut max_sum = i32::MIN;

    let mut max = i32::MIN;
    
    for i in 0..len {
        let n = array[i];
        max = core::cmp::max(max, n);

        if (sum + n) < 0 {
            sum = 0;
        } else {
            sum += n;
        }
        
        max_sum = core::cmp::max(sum, max_sum);
    }

    if max_sum == 0 {
        return max;
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subarray1() {
        let result = max_subarray1(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_max_subarray2() {
        let result = max_subarray2(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, 6);
    }

}