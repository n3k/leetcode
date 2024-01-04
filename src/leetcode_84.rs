//! Leetcode 84: Largest Rectangle in Histogram
//! 
//! Given n non-negative integers representing the 
//! histogram's bar height where the width 
//! of each bar is 1, find the area of largest
//! rectangle in the histogram


/// Solution: we go element by element and calculate the rectangles
/// that are formed with the previous elements of the sequence
/// At each step, we compare with the currently known largest one 
/// and swap if necessary.
fn largest_rectangle_1(heights: &[usize]) -> usize {
    
    let len = heights.len();
    if len == 0  {
        return 0;
    }

    if len == 1 {
        return heights[0];
    }

    let mut largest_rectangle = heights[0];
    
    for i in 1..len {
        let mut bar = heights[i];
        
        // Check if the single current bar alone is largest
        // than what was previously calculated
        largest_rectangle = core::cmp::max(bar, largest_rectangle);

        let mut j = 2;
        for k in (0..i).rev() {
            let cur_height = heights[k];
            if cur_height == 0 {
                break;
            }
            
            let top_height = core::cmp::min(cur_height, bar);
            let cur_rect = top_height * j;
            
            largest_rectangle = core::cmp::max(cur_rect, largest_rectangle);
            
            bar = top_height;
            j   += 1;
        }
    }
    
    largest_rectangle
}



fn largest_rectangle_2(heights: &[usize]) -> usize {
    
    let len = heights.len();
    if len == 0  {
        return 0;
    }

    if len == 1 {
        return heights[0];
    }

    let mut largest_rectangle = heights[0];

    let mut last_zero_height_idx = 0usize;

    let mut phase_min_idx = 0usize;
    let mut phase_min_height     = heights[0];

    for i in 1..len {
        let mut cur_height = heights[i];
        
        // Check if the single current bar alone is largest
        // than what was previously calculated
        largest_rectangle = core::cmp::max(cur_height, largest_rectangle);


        if cur_height == 0 {
            // New Phase
            last_zero_height_idx = i;
            continue;
        }

        // Phases are divided by Bars with Zero Height
        if cur_height <= phase_min_height {
            phase_min_height     = cur_height;
            phase_min_idx        = i;
        }

        

        let mut j = 2;
        let mut k = i - 1;

        loop {
            // If we got to this point, there is no need to keep going 
            // left, just multiply the min height for the range and check for the max
            if k == phase_min_idx {
                let phase_min_rect = {
                    if last_zero_height_idx == 0 {
                        phase_min_height * (i - last_zero_height_idx + 1)
                    } else {
                        phase_min_height * (i - last_zero_height_idx)
                    }
                };
                    
                largest_rectangle = core::cmp::max(phase_min_rect, largest_rectangle);   
                break; 
            }

            let prev_height = heights[k];

            let top_height = core::cmp::min(prev_height, cur_height);
            let cur_rect = top_height * j;
            
            largest_rectangle = core::cmp::max(cur_rect, largest_rectangle);
            
            cur_height = top_height;
            j   += 1;

            if k == last_zero_height_idx {
                break;
            }
            
            k = k - 1;
        }
    }
    
    largest_rectangle
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rect_1() {
        let result = largest_rectangle_1(&[2, 1, 5, 6, 2, 3]);
        assert_eq!(result, 10);
    }


    #[test]
    fn test_largest_rect_2() {
        let result = largest_rectangle_2(& [4,2,0,3,2,5]);
        assert_eq!(result, 6);
    }
   
}