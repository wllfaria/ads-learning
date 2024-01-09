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
 * @return {number[]}
 */
var averageOfLevels = function (root) {
    if (!root) return [];

    const queue = [root];
    const avg = [];

    avg.push(queue[0].val);

    while (queue.length != 0) {
        let size = queue.length;
        let localAvg = [];
        for (let i = 0; i < size; ++i) {
            let n = queue.shift();
            if (n.left) {
                queue.push(n.left);
                localAvg.push(n.left.val);
            }
            if (n.right) {
                queue.push(n.right);
                localAvg.push(n.right.val);
            }
        }
        if (!localAvg.length) continue;
        let sum = localAvg.reduce((acc, x) => (acc += x), 0);
        sum /= localAvg.length;
        avg.push(sum);
    }
    return avg;
};
