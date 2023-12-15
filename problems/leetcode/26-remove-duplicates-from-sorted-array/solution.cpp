#include <iostream>
#include <vector>
#include <set>
using namespace std;

class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        set<int> s;
        int index = 0;

        for (int i = 0; i < nums.size(); i++) {
            if (!s.count(nums[i])) {
                s.insert(nums[i]);
                nums[index] = nums[i];
                index++;
            }
        }

        return index;
    }
};

int main() {
    vector<int> nums = { 1, 1, 2 };
    Solution solution;
    int k = solution.removeDuplicates(nums);
    cout << k;
}
