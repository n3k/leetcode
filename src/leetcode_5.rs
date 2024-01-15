//! 5. Longest Palindromic Substring
//! Given a string s, return the longest palindromic
//! substring in s.



mod Solution {

    pub fn is_palindrome(s: &str) -> bool {
        let len = s.len();
        let mut cmps = 0;
        // [x, x, x]
        //  
        //println!("Len: {}", len);

        if len & 1 == 1 {
            loop {
                //println!("cmps: {} - len/2 =  {}", cmps, len>>1);
                if cmps == ((len - 1) >> 1)  {
                    break;
                }
               
                if s.chars().nth(cmps).unwrap() != 
                    s.chars().nth(len - cmps - 1).unwrap() {
                    return false;
                }
    
                cmps += 1;
            }
    
            return true;
        } else {
            loop {
                //println!("cmps: {} - len/2 =  {}", cmps, len>>1);
                if cmps == (len >> 1)  {
                    break;
                }
               
                if s.chars().nth(cmps).unwrap() != 
                    s.chars().nth(len - cmps - 1).unwrap() {
                    return false;
                }
    
                cmps += 1;
            }
    
            return true;
        }
        
    }

    /// Slow ...
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();

        if len == 0 {
            return "".into();
        }

        if len == 1 { 
            return s;
        }

        let mut l = 0usize;
        let mut r = 1usize;

        /// Test for window sizes 
        let mut w_size = len;
        let mut iter = 0;
        // [x , x, x, x, x, x, x , x]
        loop {
            l = iter;
            r = l + w_size;

            println!("wsize: {} - iter: {}", w_size, iter);
            if is_palindrome(&s[l..r]) {
                return s[l..r].into();
            }

            iter += 1;
            if iter + w_size > len {
                iter = 0;
                w_size -= 1;
            }

            if w_size == 1 {
                return s.chars().nth(0).unwrap().into();
            }
        }
    }

    // Try spliting the string and comparing?.. log n ?

    pub fn longest_palindrome_2(s: String) -> String {

        let len = s.len();
    
        if len == 0 {
            return "".into();
        }
    
        if len == 1 { 
            return s;
        }
        
        let mut lmax = 0;
        let mut rmax = 0;
        let mut longest = 1i32;
        
        //println!("--> s = {}", &s);

            for i in 0..len {
                let mut l = i as i32;
                let mut r = i as i32;
    
                while l >= 0 && (r as usize) < len &&
                    s.chars().nth(l as usize).unwrap() == s.chars().nth(r as usize).unwrap()
                {
                    //println!("l={} r={}", l, r);
                    if (r - l + 1) > longest {
                        longest = r - l + 1;
                        lmax = l as usize;
                        rmax = r as usize;
                        //println!("Setting longest: {}, l={} r={}", longest, l, r);
                    }
    
                    l = l - 1;
                    r = r + 1;
                }

          
                let mut l = i as i32;
                let mut r = (i + 1) as i32;
    
                while l >= 0 && (r as usize) < len &&
                    s.chars().nth(l as usize).unwrap() == s.chars().nth(r as usize).unwrap()
                {
                    //println!("l={} r={}", l, r);
                    if (r - l + 1) > longest {
                        longest = r - l + 1;
                        lmax = l as usize;
                        rmax = r as usize;
                    }
    
                    l = l - 1;
                    r = r + 1;
                }
            }
        
    
        s[lmax..=rmax].into()
        
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(Solution::is_palindrome("solos".into()), true);
        assert_eq!(Solution::is_palindrome("solxlos".into()), true);
        assert_eq!(Solution::is_palindrome("bb".into()), true);
        assert_eq!(Solution::is_palindrome("solxos".into()), false);
        assert_eq!(Solution::is_palindrome("ab".into()), false);
        
        assert_eq!(Solution::longest_palindrome_2("babad".into()), "bab");
        assert_eq!(Solution::longest_palindrome_2("cbbd".into()), "bb");
        assert_eq!(Solution::longest_palindrome_2("cbb".into()), "bb");
        assert_eq!(Solution::longest_palindrome_2("bb".into()), "bb");
        assert_eq!(Solution::longest_palindrome_2("aaabaaaa".into()), "aaabaaa");
        // assert_eq!(Solution::longest_palindrome_2("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabcaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into()), "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    }
}