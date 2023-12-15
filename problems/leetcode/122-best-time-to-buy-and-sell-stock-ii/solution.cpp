#include<iostream>
#include<vector>

using namespace std;

class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int profit = 0;
        int i = 0;
        int buy, sell;

        while (i < prices.size() - 1) {
            while (i < prices.size() - 1 && prices[i + 1] <= prices[i]) {
                i++;
            }
            buy = prices[i];
            while (i < prices.size() - 1 && prices[i + 1] > prices[i]) {
                i++;
            }
            sell = prices[i];
            profit += sell - buy;
        }

        return profit;
    }
};

/*
 * I couldn't solve this myself, so I got this from the solutions tab.
 * This uses the Valley/Peak approach (which i didn't knew about at this time)
 * This algorithm basically looks at the next number and if it is lower than the current number
 * we buy it, then, we look at the next number and if it is higher, we sell at that price
 * else we sell at the same price we bought for 0 profit, then we just keep track of the proffit
 */

int main() {
    vector<int> prices = { 7, 1, 5, 3, 6, 4 };
    Solution solution;
    int k = solution.maxProfit(prices);
    cout << k;
}
