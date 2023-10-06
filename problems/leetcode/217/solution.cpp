#include <iostream>
#include <map>
#include <vector>

class Solution {
public:
    bool containsDuplicate(std::vector<int> &nums) {
        std::map<int, bool> ans;

        for (int n : nums) {
            if (ans.find(n) != ans.end()) {
                return true;
            }
            ans.insert({n, true});
        }

        return false;
    }
};

// Could've answered using a set, haven't thought about it.
// Sets can't have duplicates, so if the set has a smaller size than the nums vector. it had duplicates
// return nums.size() > std::set<int>(nums.begin(), nums.end()).size()
