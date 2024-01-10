const traverse = (left, right) => {
    if (!left && !right) return true;
    if (!left || !right) return false;
    if (left.val !== right.val) return false;

    return traverse(left.left, right.right) && traverse(left.right, right.left);
};

var isSymmetric = function (root) {
    if (!root) return false;
    return traverse(root.left, root.right);
};

const root_a = {
    val: 1,
    left: { val: 2, left: { val: 3 }, right: { val: 4 } },
    right: { val: 2, left: { val: 4 }, right: { val: 3 } },
};

const root_b = {
    val: 1,
    left: { val: 2, right: { val: 3 } },
    right: { val: 2, right: { val: 3 } },
};

console.log(isSymmetric(root_a));
console.log(isSymmetric(root_b));
