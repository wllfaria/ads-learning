#include <iostream>
#include <vector>

class Solution {
public:
    std::vector<int> twoSum(std::vector<int> &numbers, int target) {
        int i = 0;
        int j = numbers.size() - 1;

        while (i <= j) {
            if (numbers.at(i) + numbers.at(j) == target)
                break;
            if (numbers.at(i) + numbers.at(j) < target) {
                i++;
                continue;
            }
            if (numbers.at(i) + numbers.at(j) > target) {
                j--;
                continue;
            }
        }

        std::vector<int> result{i + 1, j + 1};
        return result;
    }
};

int main() {
    std::vector<int> numbers{2, 7, 11, 15};
    Solution solution;
    std::vector<int> result = solution.twoSum(numbers, 9);
    std::cout << result.at(0) << result.at(1) << std::endl;
}
