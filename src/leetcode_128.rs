
//! Leetcode 128: Longest Consecutive Sequence
//! 
//! Given an unsorted array of integers, find the length
//! of the longest consecutive sequence 

use std::collections::HashSet;
use std::iter::FromIterator;

/// Returns a new array with the longest sequence
/// and the size of the sequence
fn lgs_sorted2(mut array: Vec<i64>) -> (Vec<i64>, usize) {

    let len = array.len();
    
    if len < 2 {
        return (array, len);
    }

    // First step is to sort the array
    array.sort();

    let mut prev = array[0];
    
    let mut longest_start_idx = 0;
    let mut longest_end_idx = 0;
    let mut longest_len = 1;

    let mut cur_start_idx = 0;
    let mut cur_end_idx = 0;
    let mut cur_len = 1;



    for i in 1..len {
        let num = array[i];
        
        if num == prev {
            continue;
        }

        if num - 1 == prev {
            cur_len += 1;
            cur_end_idx = i; 
        } else {
            if cur_len > longest_len {
                longest_start_idx = cur_start_idx;
                longest_end_idx   = cur_end_idx;
                longest_len = cur_len;
            }

            cur_start_idx = i;
            cur_len = 1;
        }

        prev = num;
    }

    println!("Longest-Len: {}", longest_len);
    println!("start: {} - end: {}", longest_start_idx, longest_end_idx);

    let mut res = Vec::<i64>::with_capacity(longest_len);
    unsafe { res.set_len(longest_len); };

    res.copy_from_slice(&array[longest_start_idx..=longest_end_idx]);
    
    (res, longest_len)
}


fn lgs_sorted(mut array: Vec<i64>) -> usize {

    let len = array.len();
    
    if len < 2 {
        return len;
    }

    // First step is to sort the array
    array.sort();

    //println!("{:?}", array);

    let mut prev = array[0];
    
    let mut longest_len = 1;
    let mut cur_len = 1;

    for i in 1..len {
        let num = array[i];
        
        if num == prev {
            continue;
        }

        if num - 1 == prev {
            cur_len += 1;
        } else {
            if cur_len > longest_len {
                longest_len = cur_len;
            }
            cur_len = 1;
        }

        prev = num;
    }

    core::cmp::max(cur_len, longest_len)
}


/// Solution in linear time
/// The firt step is convert the array into a Set 
/// And then, for every number in the array, check if the
/// number has a left neighbor, if it doesn,t it means it is 
/// the start of a sequence, and then it starts querying the set
/// for the forward numbers increasing the current length.
/// If it does have a left neighbor, we do nothing because it is not
/// the start of a sequence.
fn lgs_set(array: Vec<i64>) -> usize {

    let len = array.len();
    
    if len < 2 {
        return len;
    }

    let set = HashSet::<i64>::from_iter(array.iter().cloned());

    let mut longest = 1usize;
    for num in array {
        // Check if num is the start of a sequence
        if !set.contains(&(num - 1)) {
            let mut cur_length = 1;
            // This means the current number doesn't have 
            // a left neightbor
            let mut x = num + 1;           
            loop {
                if set.contains(&x) {
                    cur_length += 1;
                    x += 1;
                } else {
                    break;
                }
            }    

            if cur_length > longest {
                longest = cur_length;
            }

        } 
    }

    return longest;
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Instant;
    use rand::Rng;

    #[test]
    fn long_consecutive_sequence_1() {
        let mut rng = rand::thread_rng();

        let array: Vec<i64> = (0..rng.gen_range::<usize, _>(100..1000000))
            .map(|_| {
                rng.gen_range::<i64, _>(0..1000000)
            }).collect();
        
        let array_cloned = array.clone();

        let start1 = Instant::now();
        let len1 = lgs_set(array_cloned);
        let end1 = start1.elapsed();

        let start2 = Instant::now();
        let len2 = lgs_sorted(array);
        let end2 = start2.elapsed();

        assert_eq!(len1, len2);
        
        println!("Len: {} \nTime1: {:2.5} - Time2: {:2.5}", 
            len1, end1.as_secs_f64(), end2.as_secs_f64());
        //println!("{:?}", res);   
    }


    #[test]
    fn lgs_sorted_test_1() {

        let mut v = vec![0,3,7,2,5,8,4,6,0,1];
        let len = lgs_sorted(v);
        assert_eq!(len, 9);
    }
}
