/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function (nums, target) {
    const map = {};
    const len = nums.length;

    for (let i = 0; i < len; ++i) {
        const comp = target - nums[i];
        if (map[comp] != null) return [map[comp], i];
        map[nums[i]] = i;
    }

    return [];
};

const nums_a = [2, 7, 11, 15],
    target_a = 9;
const nums_b = [3, 2, 4],
    target_b = 6;
const nums_c = [3, 3],
    target_c = 6;

console.log(twoSum(nums_a, target_a));
console.log(twoSum(nums_b, target_b));
console.log(twoSum(nums_c, target_c));
