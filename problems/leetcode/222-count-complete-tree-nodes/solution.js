var countNodes = function (root) {
    if (!root) return 0;
    if (!root.left && !root.right) return 1;
    return 1 + countNodes(root.left) + countNodes(root.right);
};

const root = {
    val: 1,
    left: { val: 2, left: { val: 4 }, right: { val: 5 } },
    right: { val: 3, left: { val: 6 } },
};

console.log(countNodes(root));
