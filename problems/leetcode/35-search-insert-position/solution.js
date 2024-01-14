var searchInsert = function (nums, target) {
    const closest = { val: Infinity, i: -1 };
    let low = 0;
    let high = nums.length;
    while (low < high) {
        const mid = Math.floor((low + high) / 2);
        if (nums[mid] === target) return mid;
        if (Math.abs(closest.val - target) > Math.abs(nums[mid] - target)) {
            closest.val = nums[mid];
            closest.i = mid;
        }
        if (nums[mid] > target) high = mid;
        if (nums[mid] < target) low = mid + 1;
    }
    if (closest.val > target) return closest.i;
    return closest.i + 1;
};

const nums = [1, 3, 5, 6];

console.log(searchInsert(nums, 2));
