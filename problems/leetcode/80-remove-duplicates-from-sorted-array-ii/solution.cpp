#include <iostream>
#include <vector>

#include <unordered_map>

using namespace std;

class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        unordered_map<int, int> map;
        int index = 0;

        for (int i = 0; i < nums.size(); i++) {
            if (!map.count(nums[i])) {
                map[nums[i]] = 1;
                nums[index] = nums[i];
                index++;
                continue;
            }
            if (map[nums[i]] < 2) {
                map[nums[i]]++;
                nums[index] = nums[i];
                index++;
            }
        }

        return index;
    }
};

int main() {
    vector<int> nums = { 1, 1, 1, 2, 2, 3 };
    Solution solution;
    int k =solution.removeDuplicates(nums);
    cout << k;
}
