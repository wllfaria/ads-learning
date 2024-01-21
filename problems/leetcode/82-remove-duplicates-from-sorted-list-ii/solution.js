class ListNode {
    constructor(val, next) {
        this.val = val === undefined ? 0 : val;
        this.next = next === undefined ? null : next;
    }
}

var deleteDuplicates = function (head) {
    if (!head) return head;
    let dummy = new ListNode(0);
    let curr = head;
    let map = {};

    while (curr.next) {
        if (curr.next.val === curr.val) {
            map[curr.val] = true;
        }
        curr = curr.next;
    }
    curr = dummy;
    while (head) {
        if (map[head.val]) {
            head = head.next;
            continue;
        }
        curr.next = new ListNode(head.val);
        curr = curr.next;
        head = head.next;
    }

    return dummy.next;
};

const head_a = {
    val: 1,
    next: {
        val: 2,
        next: {
            val: 3,
            next: {
                val: 3,
                next: {
                    val: 4,
                    next: {
                        val: 4,
                        next: {
                            val: 5,
                            next: null,
                        },
                    },
                },
            },
        },
    },
};

const head_b = {
    val: 1,
    next: {
        val: 1,
        next: {
            val: 1,
            next: {
                val: 2,
                next: {
                    val: 3,
                    next: undefined,
                },
            },
        },
    },
};

console.log(deleteDuplicates(head_a));
console.log(deleteDuplicates(head_b));
