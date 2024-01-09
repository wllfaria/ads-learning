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
var zigzagLevelOrder = function (root) {
    if (!root) return [];
    const queue = [root];
    const ans = [];

    ans.push([queue[0].val]);
    let i = 0;
    while (queue.length != 0) {
        const size = queue.length;
        const row = [];
        for (let i = 0; i < size; ++i) {
            const curr = queue.shift();
            if (curr.right) {
                row.push(curr.right.val);
                queue.push(curr.right);
            }
            if (curr.left) {
                row.push(curr.left.val);
                queue.push(curr.left);
            }
        }
        if (row.length) {
            if (i % 2 === 1) {
                row.reverse();
            }
            ans.push(row);
        }
        i++;
    }

    return ans;
};

const root = {
    val: 3,
    left: { val: 9 },
    right: { val: 20, left: { val: 15 }, right: { val: 7 } },
};

console.log(zigzagLevelOrder(root));
