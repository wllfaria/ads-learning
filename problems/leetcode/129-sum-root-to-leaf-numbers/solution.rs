struct Solution {}

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
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // we have a running sum for each branch; and we add the current node to the running sum
    // since we are doing a depth first search, we can just multiply the running sum by 10
    // and add the current node's value to it
    // so the branch 4 -> 9 -> 5 would be calculated as:
    // 1. 0 * 10 + 4
    // 2. 4 * 10 + 9
    // 3. 49 * 10 + 5 <- this is the leaf, so it would return (495)
    // then, we just sum the left and right branches together
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, r: i32) -> i32 {
        let curr = root.unwrap().clone();
        let val = curr.as_ref().borrow().val;
        if curr.as_ref().borrow().left.is_none() && curr.as_ref().borrow().right.is_none() {
            return r * 10 + val;
        }
        let mut s = 0;
        if curr.as_ref().borrow().left.is_some() {
            s += Solution::dfs(curr.as_ref().borrow().left.clone(), r * 10 + val);
        }
        if curr.as_ref().borrow().right.is_some() {
            s += Solution::dfs(curr.as_ref().borrow().right.clone(), r * 10 + val);
        }
        s
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(root, 0)
    }
}

fn main() {
    let root_a = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    }));
    let root_b = Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    }));

    assert_eq!(Solution::sum_numbers(Some(root_a)), 25);
    assert_eq!(Solution::sum_numbers(Some(root_b)), 1026);
}
