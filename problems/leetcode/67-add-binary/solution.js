/**
 * @param {string} a
 * @param {string} b
 * @return {string}
 */
var addBinary = function (a, b) {
    const aNums = a.split("");
    const bNums = b.split("");
    const sum = [];
    let carry = 0;

    while (aNums.length || bNums.length) {
        const numA = aNums.pop() || 0;
        const numB = bNums.pop() || 0;

        const s = Number(numA) + Number(numB) + carry;
        carry = 0;
        if (s > 1) carry = 1;
        sum.push(s % 2);
    }
    if (carry) sum.push(carry);
    return sum.reverse().join("");
};

const a_a = "11";
const b_a = "1";

const a_b = "1010";
const b_b = "1011";

const a_c = "1";
const b_c = "111";

console.log(addBinary(a_a, b_a));
console.log(addBinary(a_b, b_b));
console.log(addBinary(a_c, b_c));
