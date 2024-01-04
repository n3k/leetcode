//! 3. Longest Substring Without Repeating Characters
//! Given a string s, find the length of the longest 
//! substring without repeating characters.
//! 
//! 

use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    
    let bytes = s.as_bytes();
    let len = bytes.len();
    
    if len < 2 {
        return len as i32;
    }

    let mut largest = 0usize;

    let mut l = 0usize;
    let mut r = 1usize;

    let mut map = HashMap::<u8, usize>::with_capacity(128);
    map.insert(bytes[0], 0);

    loop {
        match map.insert(bytes[r], r) {
            None => {
                r += 1;

                if r == len {
                    break;
                }
            }

            Some(j) => {               
                for byte in &bytes[l..j+1] {
                    map.remove(&byte);
                }

                largest = core::cmp::max(r - l, largest);
                
                l = j + 1;
            }
        }               
    }

    core::cmp::max(r - l, largest) as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".into()), 3);        
    }
}