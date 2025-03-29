//! Encode Decode Strings
//! 
//! Design an algorithm to encode a list of strings to a single string. 
//! The encoded string is then decoded back to the original list of strings.

//! Please implement encode and decode

//! Example 1:

//! Input: ["neet","code","love","you"]

//! Output:["neet","code","love","you"]
//! 
//! 

pub fn encode(strs: Vec<&str>) -> String {
    "".into()
}

pub fn decode(strx: String) -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_list_of_strings1() {     
        assert!(
            decode(encode(vec!["neet","code","love","you"])) == 
            vec!["neet","code","love","you"]
        );
    }
}