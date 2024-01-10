const isValidBST = function (root) {
    let result = [];
    const traverse = (root, result) => {
        if (!root) return;
        traverse(root.left, result);
        result.push(root.val);
        traverse(root.right, result);
    };
    traverse(root, result);
    for (let i = 0; i < result.length; ++i) {
        if (result[i + 1] <= result[i]) {
            return false;
        }
    }
    return true;
};

const root_a = { val: 2, left: { val: 1 }, right: { val: 3 } };
const root_b = { val: 0, left: { val: null }, right: { val: 1 } };

console.log(isValidBST(root_a));
console.log(isValidBST(root_b));
