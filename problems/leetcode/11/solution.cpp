#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int maxArea(vector<int>& height) {
        int max_sum = 0;
        int i = 0;
        int j = height.size() - 1;

        while (i <= j) {
            cout << "max sum: " << max_sum << endl;
            cout << i << " " << height[i] << " " << j << " " << height[j] << endl;
            if (height[i] <= height[j]) {
                max_sum = max(max_sum, min(height[i], height[j]) * (j - i));
                i++;
            }
            else {
                max_sum = max(max_sum, min(height[i], height[j]) * (j - i));
                j--;
            }
        }

        return max_sum;
    }
};

int main() {
    vector<int> height = { 1, 8, 6, 2, 5, 4, 8, 3, 7 };
    Solution solution;
    int k = solution.maxArea(height);
    cout << k;
}
