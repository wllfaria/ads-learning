#include <iostream>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
public:
    bool canConstruct(string ransomNote, string magazine) {
        unordered_map<char, int> m;

        for (char c : magazine) {
            if (m.count(c) > 0) m[c]++;
            else m[c] = 1;
        }

        for (char c : ransomNote) {
            if (m.count(c) == 0) return false;
            if (m[c] == 0) return false;
            m[c]--;
        }

        return true;
    }
};


int main() {
    string ransomNote = "a", magazine = "ba";
    Solution solution;
    bool k = solution.canConstruct(ransomNote, magazine);
    cout << k;
}
