//! Vertical Order Traversal
//! Given the root of a binary tree, calculate the vertical order traversal of the binary tree.
//! For each node at position (row, col), its left and right children will be at positions 
//! (row + 1, col - 1) and (row + 1, col + 1) respectively. The root of the tree is at (0, 0).
//! The vertical order traversal of a binary tree is a list of top-to-bottom orderings for each 
//! column index starting from the leftmost column and ending on the rightmost column. 
//! There may be multiple nodes in the same row and same column. In such a case, sort these nodes by their values.
//! 
//! 
//! Return the vertical order traversal of the binary tree.
//! 

use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
    TreeNode {
        val,
        left: None,
        right: None
    }
    }
}


mod Solution {

    use std::collections::HashSet;

    use super::*;  

    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::<Vec<i32>>::new();
        }

        //row, column, value
        let mut vertical_order = Vec::<(i32, i32, i32)>::new();
        // row, column, value
        let mut stack = Vec::<(i32, i32, Rc<RefCell<TreeNode>>)>::new(); 
 
        let node = root.unwrap();        
        let start_position = 0;
        stack.push((start_position, start_position,  node));

        let mut col_set = HashSet::<i32>::new();

        loop {
            if let Some((cur_row, cur_col, node)) = stack.pop() {
                
                col_set.insert(cur_col);

                vertical_order.push((cur_row, cur_col, node.borrow().val));
                if let Some(right) = node.borrow().right.clone() {
                    stack.push((cur_row + 1, cur_col+1, right));
                }
                if let Some(left) = node.borrow().left.clone() {
                    stack.push((cur_row + 1, cur_col-1 ,left));
                }
            } else {
                break;
            }
        }
        
        let mut res = Vec::<Vec::<i32>>::with_capacity(col_set.len());

        for _ in 0..col_set.len() {
            res.push(Vec::new());
        }

        vertical_order.sort_by_key(|node| {
            (node.1, node.0, node.2)
        });

        let mut cur_col = vertical_order[0].1;        
        let mut k = 0;
        res[k].push(vertical_order[0].2);
        for el in vertical_order.iter().skip(1) {
            if cur_col != el.1 {
                cur_col = el.1;
                k += 1;
            }
            res[k].push(el.2)
        }
        
        return res;
    }
   
}


/*
[1,2,3,4,6,5,7]

           1
         2    3  
       4  6  5 7

 [4] [2] [1 6 5] [3] [7]
*/


#[cfg(test)]
mod tests {
    use super::{Solution::vertical_traversal, *};

    #[test]
    fn test_vertical_order_traversal_1() {    
        let node1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None
                }))),
            }))),
        })));

        vertical_traversal(node1);

        // // 0,1,null,null,2,6,3,null,null,null,4,null,5]

        // let node2 = Some(Rc::new(RefCell::new(TreeNode {
        //     val: 0,
        //     left: Some(Rc::new(RefCell::new(TreeNode {
        //         val: 1,
        //         left: None,
        //         right: Some(Rc::new(RefCell::new(TreeNode {
        //             val: 2,
        //             left: Some(Rc::new(RefCell::new(TreeNode {
        //                 val: 6,
        //                 left: None,
        //                 right: None
        //             }))),
        //             right: Some(Rc::new(RefCell::new(TreeNode {
        //                 val: 3,
        //                 left: None,
        //                 right: None
        //             })))
        //         })))
        //     }))),
        //     right: None,
        // })));

        // panic!("{:?}", vertical_traversal(node2));
        
    }
}