//! You are given an array points, an integer angle, and your location, 
//! where location = [posx, posy] and points[i] = [xi, yi] 
//! both denote integral coordinates on the X-Y plane.

//! Initially, you are facing directly east from your position. 
//! You cannot move from your position, but you can rotate. 
//! In other words, posx and posy cannot be changed. 
//! Your field of view in degrees is represented by angle, 
//! determining how wide you can see from any given view direction. 
//! Let d be the amount in degrees that you rotate counterclockwise. 
//! Then, 
//!  your field of view is the inclusive range of angles [d - angle/2, d + angle/2].
//! 
//! You can see some set of points if, for each point, the angle formed by the point, 
//! your position, and the immediate east direction from your position is in your 
//! field of view.
//! 
//! There can be multiple points at one coordinate. 
//! There may be points at your location, and you can always see these 
//! points regardless of your rotation. Points do not obstruct your vision 
//! to other points.
//! 
//! Return the maximum number of points you can see.
//! 
//! 
mod Solution {
    use core::fmt;
    use std::{cmp::max, f32::consts::PI};

    
    #[derive(Debug)]
    struct Angle(pub f32);
    impl Angle {
        pub fn new(val: f32) -> Self {
            // Normalize to [0, 360)
            let normalized = val % 360.0;
            Angle(if normalized < 0.0 { normalized + 360.0 } else { normalized })
        }

        pub fn add(&self, rhs: &Self) -> Self {
            Angle::new(self.0 + rhs.0)
        }

        pub fn sub(&self, rhs: &Self) -> Self {
            Angle::new(self.0 - rhs.0)
        }
    }

    impl fmt::Display for Angle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl PartialOrd for Angle {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            // Wrong implementation -> doesn't satisfy transitivity
            // if self.0 == other.0 {
            //     return Some(std::cmp::Ordering::Equal);
            // } else {
            //     if self.0 >= 0.0 && self.0 <= 180.0 {
            //         if other.0 > 180.0 {
            //             return Some(std::cmp::Ordering::Greater);
            //         } else {
            //             return self.0.partial_cmp(&other.0);
            //         }
            //     } else {
            //         if other.0 < 180.0 {
            //             return Some(std::cmp::Ordering::Less);
            //         } else {
            //             return self.0.partial_cmp(&other.0);
            //         }
            //     }
            // }

            Some(self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal))
        }
    }

    impl Ord for Angle {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
        }
    }

    impl PartialEq for Angle {
        fn eq(&self, other: &Self) -> bool {
            (self.0 - other.0).abs() < 1e-5
        }
    }

    impl Eq for Angle {

    }

    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        // Obtain max_x, max_y, min_x, min_y
        let mut angles = Vec::<Angle>::new();

        let loc_x = location[0] as f32;
        let loc_y = location[1] as f32;

        let mut visible_by_default = 0;

        for point in points.iter() {
            let x = point[0] as f32;
            let y = point[1] as f32; 
            
            // Count visibles by default
            if x == loc_x && y == loc_y {
                visible_by_default += 1;
                continue;
            }

            let radians = (y - loc_y).atan2(x - loc_x);
            let degrees = radians * 180.0/PI;
            angles.push(Angle::new(degrees));
        }

        //panic!("{:?}", angles);

        let mut points_in_fov = Vec::<i32>::new();


        let mut fov_angle = 0f32;
        let angle_delta = Angle::new(angle as f32);
        let mut rotation = 0;
        while (fov_angle + angle as f32) <= 360.0 {
            

            let alpha   = Angle::new((fov_angle + ((rotation + 1) * angle) as f32) / 2f32);
            let beta    = alpha.sub(&angle_delta);

            let mut visible = visible_by_default;

            for l in angles.iter() {
                if l <= &alpha && l >= &beta {
                    visible += 1;
                }
            }
            points_in_fov.push(visible);           

            fov_angle   += angle as f32;
            rotation    += 1;
        }

        

        points_in_fov.into_iter().max().unwrap_or(0)
    }


    // not working 
    pub fn visible_points_sliding(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        // Obtain max_x, max_y, min_x, min_y
        let mut angles = Vec::<Angle>::new();

        let loc_x = location[0] as f32;
        let loc_y = location[1] as f32;

        let mut visible_by_default = 0;

        for point in points.iter() {
            let x = point[0] as f32;
            let y = point[1] as f32; 
            
            // Count visibles by default
            if x == loc_x && y == loc_y {
                visible_by_default += 1;
                continue;
            }

            let radians = (y - loc_y).atan2(x - loc_x);
            let degrees = radians * 180.0/PI;
            angles.push(Angle::new(degrees));
        }

        angles.sort();

        // println!("{:?}", angles);

        let mut points_in_fov = Vec::<i32>::new();

        let mut ii = 0usize;
 
        let angle_delta = Angle::new(angle as f32);

        let initial_alpha = angle  as f32 / 2f32;
        let mut alpha   = Angle::new(initial_alpha);
        let mut beta    = alpha.sub(&angle_delta);

        while alpha.0 > initial_alpha - 0.1 {
            
            //println!("Rot: {}", rotation);
            

            let mut visible = visible_by_default;

            let mut d = 1.0;
            for l in &angles[ii..] {
                //println!("Checking L: {:?}   a:{:?}  b:{:?} ", l, alpha, beta);
                if l > &alpha {
                    d = l.0 - alpha.0;
                    break;
                }
                if l == &alpha || l == &beta {
                    visible += 1;
                }
                else if l < &alpha && l > &beta {
                    //println!(" -> true");
                    visible += 1;
                }
            }

            points_in_fov.push(visible);    

            alpha   = alpha.add(&Angle::new(d));
            beta    = alpha.sub(&angle_delta);
        }

        // println!("{:?}", points_in_fov);

        points_in_fov.into_iter().max().unwrap_or(0)
    }
}


#[cfg(test)]
mod tests {
    use super::Solution::*;

    #[test]
    fn test_visible_points1() { 
        assert_eq!(visible_points_sliding(vec![vec![1,0],vec![2,1], vec![-2, -2], vec![-2, 1]], 
            13, 
            vec![1,1]
        ), 1);

        assert_eq!(visible_points_sliding(vec![vec![2,1],vec![2,2],vec![3,3]], 
                90, 
                vec![1,1]
            ), 3);

        assert_eq!(visible_points_sliding(vec![vec![1,0],vec![2,1]], 
            13, 
            vec![1,1]
        ), 1);

        assert_eq!(visible_points_sliding(vec![vec![2,1],vec![2,2],vec![3,4],vec![1,1]], 
            90, 
            vec![1,1]
        ), 4);


        assert_eq!(visible_points_sliding(vec![vec![0,0],vec![0,2]], 
            90, 
            vec![1,1]
        ), 2);


        assert_eq!(visible_points_sliding(
            vec![
                vec![48,29],vec![3,47], vec![95,65],vec![100,23], vec![90,5], vec![38,79], vec![23,76], vec![14,53],
                vec![12,78],vec![89,5], vec![48,72],vec![39,42], vec![23,78], vec![100,93], vec![41,72], vec![35,34],
                vec![41,91],vec![83,93],vec![54,95],vec![10,33], vec![66,74], vec![14,61], vec![69,98], vec![62,72],
                vec![72,68],vec![34,59]
            ], 
            31, 
            vec![71,95]
        ), 11);
        


        
    }
}