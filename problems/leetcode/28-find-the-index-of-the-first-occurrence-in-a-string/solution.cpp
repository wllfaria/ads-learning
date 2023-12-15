#include <iostream>
#include <string>

using namespace std;

class Solution {
public:
    int strStr(string haystack, string needle) {
        return haystack.find(needle);
    }
};

int main() {
    string haystack = "sautsad";
    string needle = "sad";

    Solution solution;
    int k = solution.strStr(haystack, needle);
    cout << k;
}
