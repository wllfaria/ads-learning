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
struct BSTIterator {
    values: Vec<i32>,
}

use std::{cell::RefCell, rc::Rc};
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut values = vec![];
        BSTIterator::dfs(root, &mut values);
        BSTIterator { values }
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let curr = root.unwrap().clone();
        BSTIterator::dfs(curr.as_ref().borrow().right.clone(), arr);
        arr.push(curr.as_ref().borrow().val);
        BSTIterator::dfs(curr.as_ref().borrow().left.clone(), arr);
    }

    fn next(&mut self) -> i32 {
        self.values.pop().unwrap()
    }

    fn has_next(&self) -> bool {
        match self.values.len() {
            0 => false,
            _ => true,
        }
    }
}

fn main() {
    let root_a = Some(Rc::new(RefCell::new(TreeNode {
        val: 7,
        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 15,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
        }))),
    })));
    let mut it = BSTIterator::new(root_a);
    assert_eq!(it.next(), 3);
    assert_eq!(it.next(), 7);
    assert_eq!(it.has_next(), true);
    assert_eq!(it.next(), 9);
    assert_eq!(it.has_next(), true);
    assert_eq!(it.next(), 15);
    assert_eq!(it.has_next(), true);
    assert_eq!(it.next(), 20);
    assert_eq!(it.has_next(), false);
}
