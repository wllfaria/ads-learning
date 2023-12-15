#include <iostream>

class Solution {
public:
    bool isPalindrome(std::string s) {
        int i = 0;
        int j = s.size() - 1;

        while (i <= j) {
            if (!isalnum(s[i])) {
                i++;
                continue;
            }
            if (!isalnum(s[j])) {
                j--;
                continue;
            }
            if (tolower(s[i]) != tolower(s[j]))
                return false;
            else {
                i++;
                j--;
            }
        }

        return true;
    }
};

int main() {
    std::string s = "A man, a plan, a canal: Panama";
    Solution solution;
    bool isPalindrome = solution.isPalindrome(s);
    std::cout << isPalindrome << std::endl;
}
