//!  Rectable Overlap
//! An axis-aligned rectangle is represented as a list [x1, y1, x2, y2], 
//! where (x1, y1) is the coordinate of its bottom-left corner, 
//! and (x2, y2) is the coordinate of its top-right corner. 
//! Its top and bottom edges are parallel to the X-axis, 
//! and its left and right edges are parallel to the Y-axis.
//! 
//! Two rectangles overlap if the area of their intersection is positive. 
//! To be clear, two rectangles that only touch at the corner or edges do not overlap.
//! 
//! Given two axis-aligned rectangles rec1 and rec2, return true if they overlap,
//! otherwise return false.
//! 
//! 
//! Example 1:
//! Input: rec1 = [0,0,2,2], rec2 = [1,1,3,3]
//! Output: true
//! Example 2:
//! 
//! Input: rec1 = [0,0,1,1], rec2 = [1,0,2,1]
//! Output: false
//! Example 3:
//! 
//! Input: rec1 = [0,0,1,1], rec2 = [2,2,3,3]
//! Output: false
//!  
//! 
//! Constraints:
//! 
//! rec1.length == 4
//! rec2.length == 4
//! -109 <= rec1[i], rec2[i] <= 109
//! rec1 and rec2 represent a valid rectangle with a non-zero area.
//! 

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

//#[derive(PartialEq, Eq, Hash, Debug)]
#[derive(Copy, Clone)]
struct Point {
    pair: (i32, i32),    
    is_border: bool
}

impl Point {
    pub fn new(x: i32, y: i32, is_border: bool) -> Self {
        Self {
            pair: (x, y),
            is_border: is_border
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.pair == other.pair
    }
}

impl Eq for Point {
  
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pair.hash(state);
    }
}

fn get_points(rec: &Vec<i32>) -> HashSet<Point> {
    let x1 = rec[0];
    let y1 = rec[1];
    let x2 = rec[2];
    let y2 = rec[3];

    let mut points = HashSet::<Point>::new();
    for i in x1..=x2 {
        for j in y1..=y2 {
            let is_border = i == x1 || i == x2 || j == y1 || j == y2;                           
            points.insert(Point::new(i, j, is_border));
        }
    }
    points
}


pub fn is_rectangle_overlap2(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    
    // Get points of rec1
    let points1 = get_points(&rec1);
    let points2 = get_points(&rec2);
   
    points1.intersection(&points2).filter(|&point| {
        points1.get(point).unwrap().is_border != points2.get(point).unwrap().is_border
    }).count() > 0
}


fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    rec1[0] < rec2[2] && rec1[2] > rec2[0] && // x1 < x2' && x2 > x1'
    rec1[1] < rec2[3] && rec1[3] > rec2[1]    // y1 < y2' && y2 > y1'
}


struct Point2 {
    x: i32,
    y: i32
}

impl Point2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
}

struct Rectangle(Point2, Point2);

fn is_rectangle_overlap3(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    let ra = Rectangle(
        Point2::new(rec1[0], rec1[1]),
        Point2::new(rec1[2], rec1[3]),
    );

    let rb = Rectangle(
        Point2::new(rec2[0], rec2[1]),
        Point2::new(rec2[2], rec2[3]),
    );

    ra.0.x < rb.1.x && ra.1.x > rb.0.x &&
    ra.0.y < rb.1.y && ra.1.y > rb.0.y
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rectangle_overlap() {    
        // assert!(is_rectangle_overlap(vec![0,0,2,2], vec![1,1,3,3]) == true);
        //assert!(is_rectangle_overlap(vec![0,0,1,1], vec![1,0,2,1]) == false);
        assert!(is_rectangle_overlap(vec![-930,154,-278,985], vec![-912,362,-630,770]) == true);

        assert!(is_rectangle_overlap2(
            vec![-687153884,-854669644,-368882013,-788694078],
            vec![540420176,-909203694,655002739,-577226067]
            ) == false
        );
        
    }
}