var removeNthFromEnd = function (head, n) {
    let left = head;
    let right = head;

    for (let i = 0; i < n; ++i) right = right.next;
    if (!right) return head.next;

    while (right.next) (left = left.next), (right = right.next);
    left.next = left.next.next;
    return head;
};

const head = {
    val: 1,
    next: {
        val: 2,
        next: {
            val: 3,
            next: {
                val: 4,
                next: {
                    val: 5,
                    next: undefined,
                },
            },
        },
    },
};
const head_b = {
    val: 1,
    next: undefined,
};

const head_c = {
    val: 1,
    next: {
        val: 2,
    },
};

//console.log(removeNthFromEnd(head, 2));
//console.log(removeNthFromEnd(head_b, 2));
console.log(removeNthFromEnd(head_c, 2));
