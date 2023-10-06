/**
 * This is a solution achieved lookin in the solutions tab. It is slightly different as it don't use a map
 * but instead uses an array which is a little faster.
 */

#include <iostream>

class Solution {
public:
    bool isAnagram(std::string s, std::string t) {
        int count[26] = {0};

        for (char x : s) {
            count[x - 'a']++;
        }

        for (char x : t) {
            count[x - 'a']--;
        }

        for (int x : count) {
            if (x != 0) {
                return false;
            }
        }

        return true;
    }
};
