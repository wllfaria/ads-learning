function traverse(node, arr) {
    if (!node) return;
    traverse(node.left, arr);
    arr.push(node.val);
    traverse(node.right, arr);
}

function getMinimumDifference(root) {
    if (!root) return 0;
    const nodes = [];
    traverse(root, nodes);
    let min = Infinity;
    for (let i = 1; i < nodes.length; i++) {
        min = Math.min(min, nodes[i] - nodes[i - 1]);
    }
    return min;
}

const root_a = {
    val: 4,
    left: { val: 2, left: { val: 1 }, right: { val: 3 } },
    right: { val: 6 },
};
const root_b = {
    val: 1,
    right: { val: 3, left: { val: 2 } },
};
const root_c = {
    val: 543,
    left: { val: 384, right: { val: 445 } },
    right: { val: 652, right: { val: 699 } },
};

console.log(getMinimumDifference(root_a));
console.log(getMinimumDifference(root_b));
console.log(getMinimumDifference(root_c));
