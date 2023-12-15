#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
private:
    static bool compare(int a, int b) {
        if (__builtin_popcount(a) == __builtin_popcount(b)) return a < b;
        return __builtin_popcount(a) < __builtin_popcount(b);
    }
public:
    vector<int> sortByBits(vector<int>& arr) {
        sort(arr.begin(), arr.end(), compare);
        return arr;
    }
};

int main() {
    vector<int> arr = { 0,1,2,3,4,5,6,7,8 };
    Solution s;
    vector<int> k = s.sortByBits(arr);

    for (int i : k) cout << i << " ";
}
