//! Valid Anagram
//! 
//! Given two strings s and t, return true 
//! if t is an anagram of s, and false otherwise.
//! 

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    
    let sx = s.as_bytes();
    let tx = t.as_bytes();
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {     
    }
}