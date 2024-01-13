const seen = {};

function isHappy(n) {
    if (n === 1) return true;
    if (seen[n]) return false;
    seen[n] = true;
    n = +n
        .toString()
        .split("")
        .map((v) => v ** 2)
        .reduce((acc, v) => acc + v, 0);
    return isHappy(n, seen);
}

function isHappyWhile(n) {
    const seen = {};
    while (n !== 1) {
        if (seen[n]) return false;
        seen[n] = true;
        n = +n
            .toString()
            .split("")
            .map((v) => v ** 2)
            .reduce((acc, v) => acc + v, 0);
    }
    return true;
}

console.log(isHappy(19));
console.log(isHappy(10));
