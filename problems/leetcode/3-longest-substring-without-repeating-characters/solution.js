var lengthOfLongestSubstring = function (s) {
    const seen = {};
    let max = 0;

    let left = 0;
    let right = 0;

    while (right < s.length) {
        const c = s[right];
        seen[c] ? seen[c]++ : (seen[c] = 1);

        while (seen[c] > 1) {
            const l = s[left];
            seen[l]--;
            left++;
        }

        max = Math.max(max, right - left + 1);

        right++;
    }

    return max;
};

const s1 = "abcabcbb";
const s2 = "bbbbb";
const s3 = "pwwkew";

console.log(lengthOfLongestSubstring(s1));
console.log(lengthOfLongestSubstring(s2));
console.log(lengthOfLongestSubstring(s3));
