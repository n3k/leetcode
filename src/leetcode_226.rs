//! Invert Binary Tree
//!  
//! Given the root of a binary tree, invert the tree, and return its root.

use std::cell::RefCell;
use std::rc::Rc;

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

    use std::collections::VecDeque;

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            let mut queue = VecDeque::<Rc::<RefCell::<TreeNode>>>::new();
            queue.push_back(root.clone());
                    
            'next_node: loop {
                if let Some(node) = queue.pop_front() {
                    let aux_l = node.borrow_mut().left.take();
                    let aux_r = node.borrow_mut().right.take();

                    if let Some(left) = aux_l {
                        queue.push_back(left.clone());
                        node.borrow_mut().right =  Some(left);
                    } else {
                        node.borrow_mut().right =  None;
                    }

                    if let Some(right) = aux_r {
                        queue.push_back(right.clone());
                        node.borrow_mut().left =  Some(right);
                    } else {
                        node.borrow_mut().left =  None;
                    }                    
                    
                } else {
                    break 'next_node;
                }
            }
            Some(root)
        } else {
            None
        }        
    }
   
}



#[cfg(test)]
mod tests {
    use std::cell::Ref;

    use super::{Solution::invert_tree, *};

    #[test]
    fn test_inver_tree1() {    
        let node1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left:  Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None }))),
                right: Some(Rc::new(RefCell::new(TreeNode { val: 3, left: None, right: None }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left:   Some(Rc::new(RefCell::new(TreeNode { val: 6, left: None, right: None }))),
                right:  Some(Rc::new(RefCell::new(TreeNode { val: 9, left: None, right: None }))),
            }))),
        })));

        let x = invert_tree(node1);
        println!("{:?}", x);
        
    }
}