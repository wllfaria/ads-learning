var invertTree = function (root) {
    const traverse = (root) => {
        if (!root) return;
        const temp = root.left;
        root.left = root.right;
        root.right = temp;

        traverse(root.left);
        traverse(root.right);
    };
    traverse(root);
    return root;
};

const root = {
    val: 4,
    left: { val: 2, left: { val: 1 }, right: { val: 3 } },
    right: { val: 7, left: { val: 6 }, right: { val: 9 } },
};

console.log(invertTree(root));
