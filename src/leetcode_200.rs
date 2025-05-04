//! Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), 
//! return the number of islands.

//! An island is surrounded by water and is formed by connecting adjacent 
//! lands horizontally or vertically. You may assume all four edges 
//! of the grid are all surrounded by water.
//! 


mod Solution {
    use std::collections::{HashMap, HashSet};

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        // This keeps an association between a coordinate and an `island number`
        let mut island_cords = HashMap::<(usize, usize), u32>::new();

        for row in 0..grid.len() {
            for col in  0..grid[row].len() {
                // If the position (row,col) is a 1, then check 
                // for an existent island by checking for prev and upper
                if grid[row][col] == '1' {
                    // Check UP by looking into the previous row
                    let island_up = if row > 0 {
                        let prev_row = row - 1;
                        if let Some(cords) = island_cords.get(&(prev_row, col)) {
                            Some(*cords)
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    // Check prev by looking into previous Col
                    let island_prev = if col > 0 {
                        let prev_col = col - 1;
                        if let Some(cords) = island_cords.get(&(row, prev_col)) {
                            Some(*cords)
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    match (island_up, island_prev) {
                        (None, None) => {
                            // Create new island
                            let island_id = island_cords.len();
                            island_cords.insert((row, col), island_id as u32);
                        }
                        (Some(island_up), None) => {
                            // Merge with island up
                            island_cords.insert((row, col), island_up);
                        },
                        (None, Some(island_prev)) => {
                            // Merge with island prev
                            island_cords.insert((row, col), island_prev);
                        }
                        (Some(island_up), Some(island_prev)) => {
                            // This one is merging two islands                             
                            if island_up == island_prev {
                                island_cords.insert((row, col), island_up);
                            } else {
                                // If the island ID is different we need to merge them all together
                                // We merge the all the island_prev into island_up
                                for (_cords, island_id) in island_cords.iter_mut() {
                                    if *island_id == island_prev {
                                        *island_id = island_up;
                                    }
                                }

                                island_cords.insert((row, col), island_up);
                            }

                        }
                    }
                }
            }
        }

        let mut islands = HashSet::<u32>::new();
        for (_cords, island_id) in island_cords.iter() {
            islands.insert(*island_id);
        } 

        islands.len() as i32
    }   
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_islands() {
        assert_eq!(
            Solution::num_islands(
                vec![
                    vec!['1','1','1','1','0'],
                    vec!['1','1','0','1','0'],
                    vec!['1','1','0','0','0'],
                    vec!['0','0','0','0','0']
                ]              
            ), 
            1
        );

        assert_eq!(
            Solution::num_islands(
                vec![
                    vec!['0','1','1','1','0'],
                    vec!['1','1','0','1','0'],
                    vec!['0','0','0','0','0'],
                    vec!['0','0','0','0','0']
                ]              
            ), 
            1
        );

        assert_eq!(
            Solution::num_islands(
                vec![
                    vec!['1','1','0','1','1'],
                    vec!['0','1','0','1','0'],
                    vec!['0','1','1','1','0'],
                    vec!['0','0','0','0','0']
                ]              
            ), 
            1
        );
       
    }
}