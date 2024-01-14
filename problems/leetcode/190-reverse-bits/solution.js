/**
 * I have no idea how to do bit manipulation more than just doubling the number
 * I have to learn it properly
 */

var reverseBits = function (n) {
    let res = 0;
    let pow = 31;
    while (n > 0) {
        let rightMost = n & 1;
        res = res + (rightMost << pow);
        pow--;
        n = n >>> 1;
    }
    return res >>> 0;
};

console.log(reverseBits(43261596));
console.log(reverseBits(4294967293));
