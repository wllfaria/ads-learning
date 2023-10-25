#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    void merge(vector<int> &nums1, int m, vector<int> &nums2, int n) {
        for (int i = 0; i < n; i++)
            nums1[i + n] = nums2[i];

        for (int i = 0; i < nums1.size(); i++) {
            for (int j = 0; j < nums1.size(); j++) {
                if (nums1[i] < nums1[j]) {
                    int tmp = nums1[j];
                    nums1[j] = nums1[i];
                    nums1[i] = tmp;
                }
            }
        }
    }
};

int main() {
    vector<int> nums1 = {1, 2, 3, 0, 0, 0};
    vector<int> nums2 = {2, 5, 6};
    Solution solution;
    solution.merge(nums1, 3, nums2, 3);

    for (int i = 0; i < nums1.size(); i++) {
        cout << nums1[i] << " ";
    }
}
