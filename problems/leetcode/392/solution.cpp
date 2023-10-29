#include <iostream>
#include <string>

using namespace std;

class Solution {
public:
    bool isSubsequence(string s, string t) {
        if (s.size() == 0) return true;
        if (t.size() < s.size()) return false;

        int i = 0;
        int j = 0;

        while (i < t.size()) {
            if (tolower(t[i]) == tolower(s[j])) {
                i++;
                j++;
                continue;
            }
            i++;
        }

        return j == s.size() ? true : false;
    }
};

int main() {
    string s = "abc";
    string x = "axc";
    string t = "ahbgdc";

    Solution solution;
    bool k = solution.isSubsequence(s, t);
    cout << k;
}
