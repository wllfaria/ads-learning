#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution{
public:
    void rotate(vector<int>&nums, int k) {
        reverse(nums.begin(), nums.begin() + (nums.size() - k - 1));
        reverse(nums.begin() + (nums.size() - k - 1), nums.end());
        reverse(nums.begin(), nums.end());
    }
};


// I couldn't solve this one. The above solution comes from the solutions tab.
// First, we address the corner case when we don't have enough numbers to shift.
// k %= nums.size();
// This makes k = 0 when the size of the array is less than K
// Then, reverse the numbers from 0 to k;
// [1,2,3,4,5,6,7] -> [4,3,2,1,5,6,7]
// Then, reverse the last k numbers (nums.size() - k)
// [4,3,2,1,5,6,7] -> [4,3,2,1,7,6,5]
// Then just reverse the entire vector
// [4,3,2,1,7,6,5] -> [5,6,7,1,2,3,4]

int main() {
    vector<int> nums = { 1, 2, 3, 4, 5, 6, 7 };
    int k = 3;
    Solution solution;
    solution.rotate(nums, k);

    cout << endl;
    for (int item : nums) {
        cout << item << " ";
    }
}
