const hasPathSum = function (root, targetSum) {
    if (!root) return false;
    if (!root.left && !root.right) {
        return targetSum === root.val;
    }
    return (
        hasPathSum(root.left, targetSum - root.val) ||
        hasPathSum(root.right, targetSum - root.val)
    );
};

const root = {
    val: 5,
    left: { val: 4, left: { val: 11, left: { val: 7 }, right: { val: 2 } } },
    right: { val: 8, left: { val: 13 }, right: { val: 4, right: { val: 1 } } },
};

console.log(hasPathSum(root, 22));
