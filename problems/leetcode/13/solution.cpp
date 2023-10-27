#include <iostream>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
public:
    int romanToInt(string s) {
        int sum = 0;
        unordered_map<char, int> map;
        map['I'] = 1;
        map['V'] = 5;
        map['X'] = 10;
        map['L'] = 50;
        map['C'] = 100;
        map['D'] = 500;
        map['M'] = 1000;

        for (int i = 0; i < s.size(); i++) {
            if (map[s[i]] < map[s[i + 1]]) sum -= map[s[i]];
            else sum += map[s[i]];
        }

        return sum;
    }
};

int main() {
    //string s = "III";
    //string s = "LVIII";
    //string s = "MCMXCIV";
    string s = "DCXXI";
    Solution solution;
    int k =solution.romanToInt(s);
    cout << k;
}
