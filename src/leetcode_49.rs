//! Group Anagrams
//! 
//! Given an array of strings strs, group the anagrams together.
//!  You can return the answer in any order.
//! 
//! Input: strs = ["eat","tea","tan","ate","nat","bat"]
//! Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
//! 
//! 

pub fn is_anagram(s: &String, t: &String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    
    let sx = s.as_bytes();
    let tx = t.as_bytes();

    // Frequency array - each 
    let mut array_zero = [0; 26];

    for jj in 0..s.len() {
        let i = (sx[jj] - 0x61) as usize;
        let j = (tx[jj] - 0x61) as usize;
        array_zero[i] += 1;
        array_zero[j] -= 1;
    }
        
    if [0; 26].eq(&array_zero) {
        return true;
    }

    return false;
}

pub fn group_anagrams(mut strs: Vec<String>) -> Vec<Vec<String>> {
    let mut grouped_anagrams = Vec::<Vec::<String>>::new();

    if strs.len() == 0 {
        return grouped_anagrams;
    } else if strs.len() == 1 {
        grouped_anagrams.push(strs);
        return grouped_anagrams;    
    }
    
    grouped_anagrams.push(vec![strs.pop().unwrap()]);
   
    // Drain the vector of strings one by one
    'next_str: while let Some(current) = strs.pop() {
        for group in grouped_anagrams.iter_mut() {
            if is_anagram(&group[0], &current) {
                group.push(current);
                continue 'next_str;
            }
        }
        // reaching here means the string did not belong to any existent group
        grouped_anagrams.push(vec![current]);
    }

    grouped_anagrams
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x() {
    }
}