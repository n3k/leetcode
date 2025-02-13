//! 111. Minimum Depth of Binary Tree
//! 
//! Given a binary tree, find its minimum depth.
//! 
//!  The minimum depth is the number of nodes along the shortest 
//!  path from the root node down to the nearest leaf node.
//! 
//!  Note: A leaf is a node with no children.
//! 
//! 
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

    use super::*; 

    use std::{collections::VecDeque, i32};

     

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::<(Rc<RefCell<TreeNode>>, i32)>::new();

        if root.is_none() {
            return 0;
        }

        let mut minimum_depth = i32::MAX;        
        queue.push_back((root.unwrap(), 1)); // count 1 node already

        'next_node: loop {
            if let Some((node, number_of_nodes)) = queue.pop_front() {                
                
                if minimum_depth != i32::MAX && number_of_nodes > minimum_depth {
                    continue 'next_node;
                }
                
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    minimum_depth = std::cmp::min(minimum_depth, number_of_nodes);
                    continue 'next_node;
                }

                if let Some(left) = node.borrow().left.clone() {
                    queue.push_back((left, number_of_nodes + 1));
                }

                if let Some(right) = node.borrow().right.clone() {
                    queue.push_back((right, number_of_nodes + 1));
                }


            } else {
                break 'next_node;
            }
        }


        minimum_depth   
    }
   
}



#[cfg(test)]
mod tests {
    use super::{Solution::min_depth, *};

    #[test]
    fn test_min_depth_1() {    
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

        min_depth(node1);

        
    }
}