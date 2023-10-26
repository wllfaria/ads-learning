#include <iostream>
#include<vector>

using namespace std;

/*
* This works but it is by no means effective. larger lists take forever to compute.
*/

class Solution {
    struct Node {
        int index;
        vector<struct Node*> connected;
    };
public:
    struct Node* make_tree(struct Node* root, vector<int>& nums) {
        int i = root->index;

        if (nums[i] == 0) return root;

        while (i < root->index + nums[root->index]) {
            i++;
            if (i > nums.size() - 1) break;
            struct Node* node = new Node;
            node->index = i;
            root->connected.push_back(node);
        }

        for (struct Node* node : root->connected) {
            make_tree(node, nums);
        }

        return root;
    }

    int get_max_index_on_tree(struct Node* root, int max_index) {
        if (root->connected.size() == 0) return max(max_index, root->index);

        for (struct Node* node : root->connected) {
            max_index = max(max_index, get_max_index_on_tree(node, max_index));
        }

        return max_index;
    }

    bool canJump(vector<int>& nums) {
        struct Node* root = new Node;
        int max_index = 0;
        root->index = 0;

        make_tree(root, nums);
        max_index = get_max_index_on_tree(root, 0);

        return (max_index == nums.size() - 1) ? true : false;
    }
};

int main() {
    vector<int> nums = { 8,2,4,4,4,9,5,2,5,8,8,0,8,6,9,1,1,6,3,5,1,2,6,6,0,4,8,6,0,3,2,8,7,6,5,1,7,0,3,4,8,3,5,9,0,4,0,1,0,5,9,2,0,7,0,2,1,0,8,2,5,1,2,3,9,7,4,7,0,0,1,8,5,6,7,5,1,9,9,3,5,0,7,5};
    Solution solution;
    bool t = solution.canJump(nums);
    cout << t;
}
