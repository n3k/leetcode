//! Top K Frequent Elements
//! 
//! 
//!  Given an integer array nums and an integer k, return the k most frequent elements.
//!  You may return the answer in any order.
//! 

use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

/// O(n log n)
pub fn top_k_frequent(mut nums: Vec<i32>, k: i32) -> Vec<i32> {

    assert!(nums.len() >= k as usize);

    let mut map = HashMap::<i32, u32>::new();
    for num in nums.iter() {
        if map.contains_key(num) {
            let frq = map.get_mut(num).unwrap();
            *frq += 1;
        } else {
            map.insert(*num, 1);
        }
    }

    let mut vec: Vec<_> = map.iter().collect();
    vec.sort_by_key(|&(_, v)| std::cmp::Reverse(v));

    let mut res = Vec::<i32>::new();
    for jj in 0..k as usize {
        res.push(*vec[jj].0 as i32)
    }

    res
}

// O(n log k) 
pub fn top_k_frequent2(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq_map = HashMap::new();
    
    // Step 1: Count frequencies (O(n))
    for num in nums {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    // Step 2: Use a min-heap to keep the top k elements (O(n log k))
    let mut min_heap = BinaryHeap::new();
    
    // Use a min heap so that the smallest frequency is always at the top
    // Each time we push, we check also pop the smallest from the top
    for (&num, &freq) in freq_map.iter() {
        min_heap.push(Reverse((freq, num))); // Reverse makes it a min-heap
        if min_heap.len() > k as usize {
            min_heap.pop(); // Maintain only k elements
        }
    }

    // Step 3: Extract results from heap (O(k log k))
    min_heap.into_iter().map(|Reverse((_, num))| num).collect()
}

/// O (n) => Using BucketSort
pub fn top_k_frequent3(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq_map = HashMap::new();
    
    // Step 1: Count frequencies (O(n))
    for num in &nums {
        *freq_map.entry(*num).or_insert(0usize) += 1;
    }
    
    let mut buckets = vec![Vec::<i32>::new(); nums.len()];

    for (num, freq) in freq_map.iter() {
        buckets[*freq - 1].push(*num);
    }

    let mut res = Vec::<i32>::new();
    
    // Iterate over the buckets in reverse (descending frequency order)
    let mut idx = buckets.len() - 1;
    'next_bucket: loop {
        let bucket = &mut buckets[idx];
        
        // If bucket is empty, go the next bucket
        if bucket.len() == 0 {
            idx -= 1;
            continue 'next_bucket;
        }

        // Append to res
        res.push(bucket.pop().unwrap());

        // If K were found then break
        if res.len() == k as usize {
            break 'next_bucket;
        }
    }
    
    res
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {     
        assert!(top_k_frequent3(vec![1,1,1,2,2,3],  2) == vec![1, 2]);
        assert!(top_k_frequent3(vec![1],  1) == vec![1]);
        assert!(top_k_frequent3(vec![-1,-1], 1) == vec![-1]);
    }
}