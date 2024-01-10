/*
 * I had to check other people solution for this one, but I really got
 * the hang of it. Basically we build the linked list from the tail recursively
 */

var flatten = function (root) {
    let head = null;
    const postOrder = (node) => {
        if (node.right) postOrder(node.right);
        if (node.left) postOrder(node.left);
        node.left = null;
        node.right = head;
        head = node;
    };

    if (root) postOrder(root);

    return head;
};

const root = {
    val: 1,
    left: { val: 2, left: { val: 3 }, right: { val: 4 } },
    right: { val: 5, right: { val: 6 } },
};
