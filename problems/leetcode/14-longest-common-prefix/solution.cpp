#include <vector>
#include <iostream>
#include <string>

using namespace std;

class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        if (strs.size() == 0) return "";
        // we assume the index 0 to be our prefix
        string prefix = strs[0];

        // we iterate over the vector from the second item
        // for each iteration we remove the last letter of the prefix if
        // the return of find() != 0. means that the prefix was not found
        // we repeat this process until the strings match and go for the next
        // or if the prefix becomes empty, we return it.
        for (int i = 1; i < strs.size(); i++) {
            while (strs[i].find(prefix, 0) != 0) {
                cout << strs[i].find(prefix, 0) << endl;
                prefix = prefix.substr(0, prefix.size() - 1);
                if (prefix == "") return "";
            }
            
        }

        return prefix;
    }
};

int main() {
    vector<string> strs = { "flower","flow","flight" };
    Solution solution;
    string k = solution.longestCommonPrefix(strs);
    cout << k;
}
