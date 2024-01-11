/**
 * @param {number[]} nums
 * @return {string[]}
 */
var summaryRanges = function (nums) {
    const ranges = [];
    let start = nums[0];
    let curr;

    for (let i = 0; i < nums.length; ++i) {
        curr = nums[i];
        if (curr + 1 === nums[i + 1]) continue;
        if (start === curr) {
            ranges.push(`${start}`);
        } else {
            ranges.push(`${start}->${curr}`);
        }
        start = nums[i + 1];
    }

    return ranges;
};

const nums_a = [0, 1, 2, 4, 5, 7];
const nums_b = [0, 2, 3, 4, 6, 8, 9];

console.log(summaryRanges(nums_a));
console.log(summaryRanges(nums_b));
