/// Definition for singly-linked list.
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

/////////////////////////////////////////////////////////////////////////////////

/// A better solution from another person
pub fn add_two_numbers_x(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add_two_numbers_helper(l1, l2, 0)
}

fn add_two_numbers_helper(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }

    let l1 = l1.unwrap_or(Box::new(ListNode::new(0)));
    let l2 = l2.unwrap_or(Box::new(ListNode::new(0)));

    let sum = l1.val + l2.val + carry;
    let mut current_node = ListNode::new(sum % 10);
    current_node.next = add_two_numbers_helper(l1.next, l2.next, sum / 10);

    Some(Box::new(current_node))
}

//////////////////////////////////////////////////////

/// My solution with ptrs xD
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
    let mut carry = false;
    

    let l3_head =  Box::new(ListNode {val: 2, next: None});
    let l3_head_ptr = Box::into_raw(l3_head);
    let mut l3p     = l3_head_ptr;

    let mut l1_ended = false;
    let mut l2_ended = false;

    let mut l1p = &l1;
    let mut l2p = &l2;

    let mut last_ptr: *mut ListNode = l3p;

    loop {
        let v1 = if let Some(node) = l1p {
            l1p = &node.next;
            node.val
        } else {
            l1_ended = true;
            0
        };

        let v2 = if let Some(node) = l2p {
            l2p = &node.next;
            node.val
        } else {
            l2_ended = true;
            0
        };

        if l1_ended && l2_ended {   
            if carry == true {
                unsafe { 
                    (*l3p).val  = 1; 
                };
            } else {
                unsafe {
                    (*last_ptr).next = None;
                }
            }
            break;
        }

        let res = v1 + v2 + carry as i32;
        let v3 = res % 10;
        carry = res > 9;

        //println!("Node val: {} - carry: {}", res, carry);

        unsafe { 
            // Assign value
            (*l3p).val  = v3; 

            // Create new node for the next iteration
            let next_node = Box::new(ListNode {val: 0, next: None});

            // Grab a pointer to it
            let aux_p = Box::into_raw(next_node);
            
            // Assign it to the next field of the current one
            (*l3p).next  = Some(Box::from_raw(aux_p));    
            
            // Update last_ptr ptr
            last_ptr = l3p;
            
            // Update l3p to the new node
            //println!("auxp: {:x}", aux_p as usize);
            l3p = aux_p;            
        }
    }

    //println!("finish");

    Some(unsafe {Box::from_raw(l3_head_ptr)})
}


// This one turned to be faster?
mod Solution {
    use super::*;
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> { 
        
        let n1 = if let Some(node) = l1 {
            node
        } else {
            Box::new(ListNode {val: 0, next: None})
        };

        let n2 = if let Some(node) = l2 {
            node
        } else {
            Box::new(ListNode {val: 0, next: None})
        };


        let mut val = n1.val + n2.val;
        let carry = val > 9;

        // println!("Adding {} + {} = {}", n1.val, n2.val, val);
        // println!("L1.Next=[{:?}]   L2.Next=[{:?}]", n1.next, n2.next);
        val = val % 10;


        match (n1.next, n2.next) {
            (Some(nx1), Some(nx2)) => {            

                let res = Some(
                    Box::new(
                        ListNode {
                            val: val, 
                            next: if carry { 
                                Solution::add_two_numbers(Some(Box::new(ListNode{ val: 1, next: None })),
                                Solution::add_two_numbers(Some(nx1), Some(nx2)))
                            } else {
                                Solution::add_two_numbers(Some(nx1), Some(nx2))
                            }
                        })
                    );            
                return res; 
            }

            (Some(nx1), None) => {
                
                return Some(
                    Box::new(
                        ListNode {
                            val: val, 
                            next: if carry {
                                Solution::add_two_numbers(Some(Box::new(ListNode{ val: 1, next: None })),
                                Solution::add_two_numbers(Some(nx1), None))
                            } else {
                                Solution::add_two_numbers(Some(nx1), None)
                            }
                        })
                    );
            }

            (None, Some(nx2)) => {
                
                return Some(
                    Box::new(
                        ListNode {
                            val: val, 
                            next: if carry {
                                Solution::add_two_numbers(Some(Box::new(ListNode{ val: 1, next: None })),
                                Solution::add_two_numbers(None, Some(nx2)))
                            } else {
                                Solution::add_two_numbers(None, Some(nx2))
                            }
                        })
                    );
            }

            (None, None) => {
                if carry {                
                    return Some(Box::new(ListNode{val: val, 
                        next: Some(Box::new(ListNode{ val: 1, next: None }))}));
                } else {                
                    return Some(Box::new(ListNode{val: val, next: None}));
                }
            }
        }
    }

}

/// Another better attempt with recursion by myself
pub fn add_two_numbers_3(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> { 
    
    let n1 = if let Some(node) = l1 {
        node
    } else {
        Box::new(ListNode {val: 0, next: None})
    };

    let n2 = if let Some(node) = l2 {
        node
    } else {
        Box::new(ListNode {val: 0, next: None})
    };


    let mut val = n1.val + n2.val;
    let carry = val > 9;

    // println!("Adding {} + {} = {} -- Carry: {}", n1.val, n2.val, val, carry);
    // println!("L1.Next=[{:?}]   L2.Next=[{:?}]", n1.next, n2.next);
    val = val % 10;

    match (n1.next, n2.next) {
        (Some(mut nx1), Some(nx2)) => {
            nx1.val = nx1.val + carry as i32;

            let res = Some(
                Box::new(
                    ListNode {
                        val: val, 
                        next: add_two_numbers_3(Some(nx1), Some(nx2))
                    })
                );            
            return res; 
        }

        (Some(mut nx1), None) => {
            nx1.val = nx1.val + carry as i32;
            return Some(
                Box::new(
                    ListNode {
                        val: val, 
                        next: add_two_numbers_3(Some(nx1), None)
                    })
                );
        }

        (None, Some(mut nx2)) => {
            nx2.val = nx2.val + carry as i32;
            return Some(
                Box::new(
                    ListNode {
                        val: val, 
                        next: add_two_numbers_3(None, Some(nx2))
                    })
                );
        }

        (None, None) => {
            if carry {                
                return Some(Box::new(ListNode{val: val, 
                    next: Some(Box::new(ListNode{ val: 1, next: None }))}));
            } else {                
                return Some(Box::new(ListNode{val: val, next: None}));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let l1 = 
            Some(
                Box::new(
                    ListNode {
                        val: 7, 
                        next: Some(Box::new(ListNode {
                            val: 2, 
                            next: None
                        }))}));

        let l2 = 
            Some(
                Box::new(
                    ListNode {
                        val: 5, 
                        next: Some(Box::new(ListNode {
                            val: 3, 
                            next: None
                        }))}));

        let result = add_two_numbers(l1, l2).unwrap();
        assert_eq!(result, Box::new(
            ListNode {
                val: 2, 
                next: Some(Box::new(ListNode {
                    val: 6, 
                    next: None
                }))}));
    }


    #[test]
    fn it_works2() {
        let l1 = 
            Some(
                Box::new(
                    ListNode {
                        val: 2, 
                        next: Some(Box::new(ListNode {
                            val: 4, 
                            next: Some(Box::new(ListNode {
                                val: 3, 
                                next: None
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
                    }))}));

        let result = add_two_numbers_3(l1, l2).unwrap();
        assert_eq!(result, Box::new(
            ListNode {
                val: 7, 
                next: Some(Box::new(ListNode {
                    val: 0, 
                    next: Some(Box::new(ListNode {
                        val: 8, 
                        next: None
                    }))
                }))}));
    }


    #[test]
    fn it_works3() {
        let l1 = 
            Some(
                Box::new(
                    ListNode {
                        val: 9, 
                        next: Some(Box::new(ListNode {
                            val: 9, 
                            next: Some(Box::new(ListNode {
                                val: 9, 
                                next: None
                            }))
                        }))}));

        let l2 = 
        Some(
            Box::new(
                ListNode {
                    val: 9, 
                    next: Some(Box::new(ListNode {
                        val: 9, 
                        next: None
                    }))}));

        let result = add_two_numbers_3(l1, l2).unwrap();
        assert_eq!(result, Box::new(
            ListNode {
                val: 9, 
                next: Some(Box::new(ListNode {
                    val: 9, 
                    next: Some(Box::new(ListNode {
                        val: 9, 
                        next: Some(Box::new(ListNode {
                            val: 9, 
                            next: None
                        }))
                    }))
                }))}));
    }
}
