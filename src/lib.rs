mod leetcode_1;
mod leetcode_2;
mod leetcode_3;
mod leetcode_5;
mod leetcode_12;
mod leetcode_31;
mod leetcode_49;
mod leetcode_53;
mod leetcode_84;
mod leetcode_111;
mod leetcode_128;
mod leetcode_167;
mod leetcode_209;
mod leetcode_217;
mod leetcode_238;
mod leetcode_242;
mod leetcode_271;
mod leetcode_347;
mod leetcode_445;
mod leetcode_523;
mod leetcode_766;
mod leetcode_836;
mod leetcode_987;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
