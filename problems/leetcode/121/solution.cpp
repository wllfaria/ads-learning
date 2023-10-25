#include<iostream>
#include<vector>

using namespace std;

class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int max_profit = 0;
        // First we instance the min price as a value higher than the maximum price in the array
        // IE: MAX Integer number
        int min_price = INT_MAX;

        // we iterate over the vector once
        for (int i = 0; i < prices.size(); i++) {
            // If the current price index is smaller than the min price, we swap it
            min_price = min(min_price, prices[i]);
            // the max profit is the larger number between the current max profit and 
            // the current price - the minimum price
            max_profit = max(max_profit, prices[i] - min_price);
        }

        return max_profit;
    }
};

int main() {
    vector<int> prices = { 7, 1, 5, 3, 6, 4 };
    Solution solution;
    int k = solution.maxProfit(prices);
    cout << k;

    /*
     * initial values are: min_price = 999..., max_profit = 0
     * next iteration (7): min_price = 7, max_profit = 0 (7-7)
     * next iteration (1): min_price = 1, max_profit = 0 (1-1)
     * next iteration (5): min_price = 1, max_profit = 4 (5-1)
     * next iteration (3): min_price = 1, max_profit = 4 (5-1 still)
     * next iteration (6): min_price = 1, max_profit = 5 (6-1)
     * last iteration (4): min_price = 1, max_profit = 5 (6-1 still)
     * return 5;
     */
}
