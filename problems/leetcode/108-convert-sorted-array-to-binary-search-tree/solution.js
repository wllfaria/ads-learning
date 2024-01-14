class TreeNode {
    constructor(val, left, right) {
        this.val = val === undefined ? 0 : val;
        this.left = left === undefined ? null : left;
        this.right = right === undefined ? null : right;
    }
}

var sortedArrayToBST = function (nums) {
    if (nums.length === 0) return null;
    if (nums.length === 1) return new TreeNode(nums[0]);
    const mid = Math.floor(nums.length / 2);
    const root = new TreeNode(nums[mid]);
    root.left = sortedArrayToBST(nums.slice(0, mid));
    root.right = sortedArrayToBST(nums.slice(mid + 1, nums.length));
    return root;
};

const nums_a = [-10, -3, 0, 5, 9];
const nums_b = [1, 3];
const nums_c = [1, 2, 3, 4, 5, 6, 7];

console.log(sortedArrayToBST(nums_a));
console.log(sortedArrayToBST(nums_b));
console.log(sortedArrayToBST(nums_c));
