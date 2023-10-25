#include <iostream>
#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
public:
    int majorityElement(vector<int>& nums) {
        unordered_map<int, int> map;
        int greatest[2] = {0,0};
        int majority = nums.size() / 2;

        for (int item : nums) {
            if (!map.count(item)) map[item] = 1;
            else map[item]++;
            if (map[item] > greatest[1]) {
                greatest[0] = item;
                greatest[1] = map[item];
            };
            if (greatest[1] > majority) break;
        }

        return greatest[0];
    }
};

/*
 * Another possible solution (and better) is to just sort the array and return the number in the middle
 * int majorityElement(vector<int>& nums) {
 *    sort(nums.begin(), nums.end());
 *    int n = nums.size();
 *    return nums[n/2];
 * }
 */

int main() {
    vector<int> nums = { 3, 2, 3 };
    Solution solution;
    int k = solution.majorityElement(nums);
    cout << k;
}
