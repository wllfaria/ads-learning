const traverse = (root, arr) => {
    if (!root) return;

    traverse(root.left, arr);
    arr.push(root.val);
    traverse(root.right, arr);
};

const kthSmallest = function (root, k) {
    const arr = [];
    traverse(root, arr);
    return arr[k - 1];
};

const root = {
    val: 5,
    left: { val: 3, left: { val: 2, left: { val: 1 } }, right: { val: 4 } },
    right: { val: 6 },
};

console.log(kthSmallest(root, 3));
