#include <iostream>

struct Node {
    int data;
    struct Node* left;
    struct Node* right;
};

struct Node* newNode(int data) {
    struct Node* node = new Node;
    node->data = data;
    node->left = NULL;
    node->right = NULL;
    return node;
}

// Depth-First Traversals

void printPreOrder(struct Node* node) {
    if (node == NULL) return;
    std::cout << node->data << " ";
    printPreOrder(node->left);
    printPreOrder(node->right);
}

void printInOrder(struct Node* node) {
    if (node == NULL) return;
    printInOrder(node->left);
    std::cout << node->data << " ";
    printInOrder(node->right);
}

void printPostOrder(struct Node* node) {
    if (node == NULL) return;
    printInOrder(node->left);
    printInOrder(node->right);
    std::cout << node->data << " ";
}

// Breadth-First Traversals

void printCurrentLevel(struct Node* node, int level) {
    if (node == NULL) return;
    if (level == 1) std::cout << node->data << " ";
    else if (level > 1) {
        printCurrentLevel(node->left, level - 1);
        printCurrentLevel(node->right, level - 1);
    }
}

int height(struct Node* node) {
    if (node == NULL) return 0;
    else {
        int lheight = height(node->left);
        int rheight = height(node->right);
        if (lheight > rheight) return lheight + 1;
        else return rheight + 1;
    }
}

void printLevelOrder(struct Node* node) {
    int h = height(node);
    int i;
    for (i = 1; i <= h; i++) printCurrentLevel(node, i);
}

int main() {
    struct Node* root = newNode(1);
    root->left = newNode(2);
    root->right = newNode(3);
    root->left->left = newNode(4);
    root->left->right = newNode(5);
    root->right->left = newNode(6);
    root->right->right = newNode(7);
    printPreOrder(root);
    std::cout << std::endl;
    printInOrder(root);
    std::cout << std::endl;
    printPostOrder(root);
    std::cout << std::endl;
    printLevelOrder(root);
    std::cout << std::endl;
}

