/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number[][]}
 */
var levelOrder = function (root) {
    if (!root) return [];
    const queue = [root];
    const ans = [];

    while (queue.length != 0) {
        let size = queue.length;
        let row = [];
        for (let i = 0; i < size; ++i) {
            const curr = queue.shift();
            row.push(curr.val);
            if (curr.left) {
                queue.push(curr.left);
            }
            if (curr.right) {
                queue.push(curr.right);
            }
        }
        ans.push(row);
    }
    return ans;
};

const root = {
    val: 1,
    left: { val: 2, left: { val: 4 } },
    right: { val: 3, right: { val: 5 } },
};

console.log(levelOrder(root));
