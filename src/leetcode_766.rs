//! Leetcode 766. Toeplitz Matrix


mod Solution {

    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        
        let n = matrix.len();    // number of rows
        let m = matrix[0].len(); // number of columns

        assert!(n > 1);

        // column walk (diagonals starting from first row)
        for start_col in 0..m {
            let mut i = 0;
            let mut j = start_col;
            let val = matrix[i][j];

            while i < n && j < m {
                if matrix[i][j] != val {
                    return false;
                }
                i += 1;
                j += 1;
            }
        }

        // row walk (diagonals starting from first column, skipping the first row to avoid double-checking)
        for start_row in 1..n {
            let mut i = start_row;
            let mut j = 0;
            let val = matrix[i][j];

            while i < n && j < m {
                if matrix[i][j] != val {
                    return false;
                }
                i += 1;
                j += 1;
            }
        }

        return true;
    }

    /// What if the matrix is stored on disk, and the memory is
    /// limited such that you can only load at most one row of the matrix into the memory at once?
    /// Keep the prev in memory
    pub fn is_toeplitz_matrix2(matrix: Vec<Vec<i32>>) -> bool {

        let mut prev = &matrix[0];
        for i in 1..matrix.len() {
            let cur = &matrix[i];            
            assert!(cur.len() == prev.len());

            for j in 1..cur.len() {
                if cur[j] != prev[j-1] {
                    return false;
                }
            }            
            prev = &matrix[i];
        }

        return true;
    }
}


#[cfg(test)]
mod tests {
    use super::{Solution::is_toeplitz_matrix, Solution::is_toeplitz_matrix2};

    #[test]
    fn test_is_toeplitz_matrix1() {    
        assert_eq!(is_toeplitz_matrix2(vec![vec![1,2,3,4],vec![5,1,2,3],vec![9,5,1,2]]), true);
        assert_eq!(is_toeplitz_matrix2(vec![vec![1, 2],vec![2,2]]), false);
        
    }
}