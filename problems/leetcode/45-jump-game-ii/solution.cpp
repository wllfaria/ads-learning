#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int jump(vector<int>& nums) {
        int currEnd = 0;
        int currFar = 0;
        int ans = 0;

        for (int i = 0; i < nums.size() - 1; i++) {
            currFar = max(currFar, i + nums[i]);
            if (currEnd == i) {
                ans++;
                currEnd = currFar;
            }
        }

        return ans;
    }
};

int main() {
    vector<int> nums = { 2,3,1,1,4 };
    Solution s;
    int k = s.jump(nums);
    cout << k;
}
