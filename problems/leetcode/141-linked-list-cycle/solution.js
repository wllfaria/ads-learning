var hasCycle = function (head) {
    if (!head) return false;
    const seen = new Set();
    while (head.next) {
        if (seen.has(head)) return true;
        seen.add(head);
        head = head.next;
    }
    return false;
};

const node = {
    val: 1,
    next: { val: 2 },
};

console.log(hasCycle(node));
