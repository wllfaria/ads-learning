use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val
                    && Solution::is_same_tree(p.left.clone(), q.left.clone())
                    && Solution::is_same_tree(p.right.clone(), q.right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}

fn main() {
    let tree_1 = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    };
    let tree_2 = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    };
    let tree_3 = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None,
    };
    let tree_4 = TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    };
    let tree_5 = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
    };
    let tree_6 = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    };

    assert_eq!(
        Solution::is_same_tree(
            Some(Rc::new(RefCell::new(tree_1))),
            Some(Rc::new(RefCell::new(tree_2)))
        ),
        true
    );
    assert_eq!(
        Solution::is_same_tree(
            Some(Rc::new(RefCell::new(tree_3))),
            Some(Rc::new(RefCell::new(tree_4)))
        ),
        false
    );
    assert_eq!(
        Solution::is_same_tree(
            Some(Rc::new(RefCell::new(tree_5))),
            Some(Rc::new(RefCell::new(tree_6)))
        ),
        false
    );
}
