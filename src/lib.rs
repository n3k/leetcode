mod leetcode_1;
mod leetcode_2;
mod leetcode_3;
mod leetcode_5;
mod leetcode_53;
mod leetcode_84;
mod leetcode_128;
mod leetcode_167;
mod leetcode_209;
mod leetcode_445;

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
