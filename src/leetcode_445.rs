//! Leetcode 445: Add Two Numbers II
//! 
//! You are given two non-empty linked lists 
//! representing two non-negative integers. 
//! The most significant digit comes first and each of 
//! their nodes contains a single digit. 
//! Add the two numbers and return the sum as a linked list.

//! You may assume the two numbers do not 
//! contain any leading zero, except the number 0 itself.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

mod Solution {
    use super::*;

    fn stack_helper(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, 
        v1: &mut Vec<i32>, v2: &mut Vec<i32>) 
    {
        match (l1, l2) {
            (Some(n1), Some(n2)) => {
                v1.push(n1.val);
                v2.push(n2.val);
                return Solution::stack_helper(n1.next, n2.next, v1, v2);
            }
            (Some(n1), None) => {
                v1.push(n1.val);                
                return Solution::stack_helper(n1.next, None, v1, v2);
            }
            (None, Some(n2)) => {
                v2.push(n2.val);                
                return Solution::stack_helper(None, n2.next, v1, v2);
            }
            (None, None) => {
                return;
            }
        }
    }

    fn add_helper(mut v1: Vec<i32>, mut v2: Vec<i32>, carry: bool, l3: Option<Box<ListNode>>) 
        -> Option<Box<ListNode>> 
    {
        match (v1.pop(), v2.pop()) {
            (Some(n1), Some(n2)) => {                
                let add = n1 + n2 + carry as i32;
                //println!("adding {} + {} + {} = {}", n1, n2, carry, add);

                return Solution::add_helper(v1, v2, add > 9, 
                Some(Box::new(ListNode {
                        val: add % 10, 
                        next: l3
                    }))
                );                
            }

            (Some(n1), None) => {
                let add = n1 + carry as i32;
                return Solution::add_helper(v1, v2, add > 9, 
                Some(Box::new(ListNode {
                        val: add % 10, 
                        next: l3
                    }))
                );
            }

            (None, Some(n2)) => {
                let add = n2 + carry as i32;
                return Solution::add_helper(v1, v2, add > 9, 
                Some(Box::new(ListNode {
                        val: add % 10, 
                        next: l3
                    }))
                );
            }

            (None, None) => {
                if carry {
                    return Some(Box::new(ListNode { val: 1, next: l3}));
                } else {
                    return l3;
                }                
            }
        }
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack1 = Vec::<i32>::new();
        let mut stack2 = Vec::<i32>::new();

        Solution::stack_helper(l1, l2, &mut stack1, &mut stack2);

        // println!("s1: {:?}", stack1);
        // println!("s2: {:?}", stack2);

        return Solution::add_helper(stack1, stack2, false, None);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2_add_two_numbers() {        
        //     [7,2,4,3]
        //       [5,6,4]
        //r:   [7,8,0,7]

        let l1 = 
            Some(
                Box::new(
                    ListNode {
                        val: 7, 
                        next: Some(Box::new(ListNode {
                            val: 2, 
                            next: Some(Box::new(ListNode {
                                val: 4, 
                                next: Some(Box::new(ListNode {
                                    val: 3, 
                                    next: None
                                }))
                            }))
                        }))}));

        let l2 = 
        Some(
            Box::new(
                ListNode {
                    val: 5, 
                    next: Some(Box::new(ListNode {
                        val: 6, 
                        next: Some(Box::new(ListNode {
                            val: 4, 
                            next: None
                        }))
                    }))
                }));

        let result = Solution::add_two_numbers(l1, l2).unwrap();
        assert_eq!(result, Box::new(
            ListNode {
                val: 7, 
                next: Some(Box::new(ListNode {
                    val: 8, 
                    next: Some(Box::new(ListNode {
                        val: 0, 
                        next: Some(Box::new(ListNode {
                            val: 7, 
                            next: None
                        }))
                    }))
                }))}));
    }
}