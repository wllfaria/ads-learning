#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

// The intuition here is to sort the array by descending order, then we can assume that the ith
// citation that has its value lower than its index is the last h-index citation

class Solution {
public:
    int hIndex(vector<int>& citations) {
        sort(citations.begin(), citations.end(), [](const int &a, const int &b) {
            return a > b;
        });

        int i = 0;
        while (i < citations.size() && citations[i] > i) {
            i++;
        }

        return i;
    }
};

int main() {
    vector<int> citations = { 3,0,6,1,5 };
    Solution s;
    int k = s.hIndex(citations);
    cout << k;
}
