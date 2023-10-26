#include <iostream>
#include<vector>

using namespace std;

/*
* I didn't discovered this answer by myself, I saw an explanation in the solutions tab;
* I didn't copy/pasted the code either, I read the explanation and applied it.
*/

class Solution {
public:
    bool canJump(vector<int>& nums) {
        int min_step = 0;

        for (int i = nums.size() - 2; i >= 0; i--) {
            min_step++;
            if (nums[i] >= min_step) min_step = 0;
        }

        return (min_step == 0) ? true : false;
    }
};

int main() {
    vector<int> nums = { 2, 2, 3, 4 };
    Solution solution;
    bool t = solution.canJump(nums);
    cout << t;
}
