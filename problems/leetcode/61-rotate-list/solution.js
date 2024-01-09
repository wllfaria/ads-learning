class ListNode {
    constructor(val, next) {
        this.val = val === undefined ? 0 : val;
        this.next = next === undefined ? null : next;
    }
}

var rotateRight = function (head, k) {
    if (!head) return head;

    let ptr = head;
    let count = 0;

    while (ptr) {
        ++count;
        ptr = ptr.next;
    }

    k = k % count;
    ptr = head;
    let prev = head;

    while (k--) {
        ptr = ptr.next;
    }

    while (ptr.next) {
        prev = prev.next;
        ptr = ptr.next;
    }

    ptr.next = head;
    head = prev.next;
    prev.next = null;
    return head;
};

function makeList(val, amount) {
    const head = new ListNode(val);
    let v = val + 1;
    let cur = head;

    while (--amount) {
        cur.next = new ListNode(v);
        v++;
        cur = cur.next;
    }

    return head;
}

const head = makeList(1, 5);

console.log(rotateRight(head, 2));
