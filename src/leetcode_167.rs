//! Leetcode 167: Two Sum II - Input Array Is Sorted
//! 
//! Given a 1-indexed array of integers numbers that is already 
//! sorted in non-decreasing order, find two numbers such that 
//! they add up to a specific target number. 
//! Let these two numbers be numbers[index1] and numbers[index2] 
//! where 1 <= index1 < index2 <= numbers.length.
//! 
//! Return the indices of the two numbers, index1 and index2, 
//! added by one as an integer array [index1, index2] of length 2.
//! 
//! The tests are generated such that there is exactly one solution.
//!  You may not use the same element twice.
//! 
//! Your solution must use only constant extra space.


fn two_sum_sorted(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let len = numbers.len();
    assert!(len >= 2);

    if len == 2 {
        return vec![1, 2];
    }

    let mut i = 0usize;
    // No nead to check for boundary because we are ensured 
    // there is a solution
    loop {
        let num1 = numbers[i];

        let mut add = 0;
        let mut j = i + 1;
        loop {
            if j == len {
                break;
            }

            add = num1 + numbers[j];
            if add >= target {
                break;
            }
            j += 1;
        }
        
        if add == target {
            return vec![i as i32 + 1, j as i32 + 1];
        }

        i += 1;
    }

    Vec::new()
}


/// Given the numbers are sorted, we can look for the number which is 
/// immediately smaller than our target. From there, we start adding
/// the numbers from the beginning of the array and check
fn two_sum_sorted2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let len = numbers.len();
    assert!(len >= 2);

    if len == 2 {
        return vec![1, 2];
    }

    loop {
        let mut right_boundary = len - 1;
        let mut num;
        loop {
            num = numbers[right_boundary];

            if num <= target {
                break;
            } 

            right_boundary -= 1;
        }

        // At this point, right_boundary is the index 
        // that points to a number equal or less than the target

        // Now we start from the beginning of the array, adding num
        let mut j = 0;
        let mut add = 0;
        loop {
            add = num + numbers[j];
            if add >= target {
                break;
            }

            j += 1;

            if j == right_boundary {
                break;
            }
        }

        if add == target {
            if j < right_boundary {
                return vec![j as i32 + 1, right_boundary as i32 + 1];
            } else {
                return vec![right_boundary as i32 + 1, j as i32 + 1];
            }
        }

        right_boundary -= 1;
    }

    Vec::new()
}

fn two_sum_sorted_constant(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let len = numbers.len();
    assert!(len >= 2);

    if len == 2 {
        return vec![1, 2];
    }

    let mut right = len - 1;
    let mut left  = 0usize;
    let mut addition;
    loop {
        addition = numbers[left] + numbers[right];
        if addition > target {
            right += 1;
        } else if addition < target {
            left += 1;
        } else {
            return vec![left as i32 + 1, right as i32 + 1];
        }
    }

    Vec::new()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_sorted() {
        let result = two_sum_sorted(vec![2,7,11,15], 9);
        assert_eq!(result, vec![1, 2]);
    }
}
