/**
 * @param {string[]} tokens
 * @return {number}
 */
var evalRPN = function (tokens) {
    let stack = [];
    let ops = {
        "+": (a, b) => a + b,
        "-": (a, b) => a - b,
        "*": (a, b) => a * b,
        "/": (a, b) => (a / b >= 0 ? Math.floor(a / b) : Math.ceil(a / b)),
    };

    for (let t of tokens) {
        if (ops[t]) {
            let top = stack.pop();
            let second = stack.pop();
            stack.push(ops[t](second, top));
        } else {
            stack.push(Number(t));
        }
    }
    return stack.pop();
};

const tokens_a = ["2", "1", "+", "3", "*"];
const tokens_b = ["4", "13", "5", "/", "+"];
const tokens_c = [
    "10",
    "6",
    "9",
    "3",
    "+",
    "-11",
    "*",
    "/",
    "*",
    "17",
    "+",
    "5",
    "+",
];

console.log(evalRPN(tokens_a));
console.log(evalRPN(tokens_b));
console.log(evalRPN(tokens_c));
