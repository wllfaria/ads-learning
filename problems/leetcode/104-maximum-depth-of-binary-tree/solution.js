var maxDepth = function (root) {
    if (!root) return 0;
    const queue = [root];
    let level = 0;

    while (queue.length) {
        let size = queue.length;
        for (let i = 0; i < size; ++i) {
            const curr = queue.shift();
            if (curr.left) queue.push(curr.left);
            if (curr.right) queue.push(curr.right);
        }
        ++level;
    }
    return level;
};

const root_a = {
    val: 3,
    left: { val: 9 },
    right: { val: 20, left: { val: 15 }, right: { val: 7 } },
};
const root_b = { val: 1, right: { val: 2 } };

console.log(maxDepth(root_a));
console.log(maxDepth(root_b));
