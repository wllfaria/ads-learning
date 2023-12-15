#include <iostream>
#include <map>

class Solution {
public:
    bool isAnagram(std::string s, std::string t) {
        std::map<char, int> count;

        for (char x : s) {
            count[x]++;
        }

        for (char x : t) {
            count[x]--;
        }

        for (auto x : count) {
            if (x.second != 0) {
                return false;
            }
        }

        return true;
    }
};

int main() {
    // std::string s = "anagram", t = "nagaram"
    std::string s = "rat", t = "car";
    Solution solution;
    bool answer = solution.isAnagram(s, t);
    std::cout << answer << std::endl;
}
