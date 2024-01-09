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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = vec![];
        let mut queue = vec![];

        match root {
            Some(node) => queue.push(node),
            None => (),
        }

        while queue.len() != 0 {
            ret.push(queue[queue.len() - 1].borrow().val);

            let mut row = vec![];

            for node in queue.iter() {
                match node.borrow().left.as_ref() {
                    Some(x) => row.push(Rc::clone(x)),
                    None => (),
                }

                match node.borrow().right.as_ref() {
                    Some(x) => row.push(Rc::clone(x)),
                    None => (),
                }
            }

            queue = row;
        }

        ret
    }
}

fn make_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode {
        val,
        left: None,
        right: None,
    }))
}

fn main() {
    let node_1 = make_node(1);
    let node_2 = make_node(2);
    node_1.borrow_mut().right = Some(node_2.clone());
    assert_eq!(Solution::right_side_view(Some(node_1)), vec![1, 2, 3]);
}
