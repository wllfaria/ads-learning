class ListNode {
    constructor(val, next) {
        this.val = val === undefined ? 0 : val;
        this.next = next === undefined ? null : next;
    }
}

/**
 * @param {ListNode} list1
 * @param {ListNode} list2
 * @return {ListNode}
 */
var mergeTwoLists = function (list1, list2) {
    if (list1 == null) return list2;
    if (list2 == null) return list1;
    if (list1.val < list2.val) {
        list1.next = mergeTwoLists(list1.next, list2);
        return list1;
    } else {
        list2.next = mergeTwoLists(list1, list2.next);
        return list2;
    }
};
const list1_a = { val: 1, next: { val: 2, next: { val: 4 } } };
const list1_b = { val: 1, next: { val: 3, next: { val: 4 } } };
const list2_a = new ListNode();
const list2_b = new ListNode();

console.log(mergeTwoLists(list1_a, list1_b));
console.log(mergeTwoLists(list2_b, list2_b));
