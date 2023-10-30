#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    static int findWeight(int n) {
        int mask = 1;
        int weight = 0;

        while (n > 0) {
            if ((n & mask) > 0) {
                weight++;
                n ^= mask;
            }
            mask <<= 1;
        }

        return weight;
    }

    static bool compare(int a, int b) {
        if (findWeight(a) == findWeight(b)) return a < b;
        return findWeight(a) < findWeight(b);
    }

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
