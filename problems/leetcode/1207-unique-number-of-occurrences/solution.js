var uniqueOccurrences = function (arr) {
    const map = {};
    for (let i = 0; i < arr.length; ++i) {
        map[arr[i]] ? map[arr[i]]++ : (map[arr[i]] = 1);
    }
    const values = Object.values(map);
    return new Set(values).size === values.length;
};

const arr_a = [1, 2, 2, 1, 1, 3];
const arr_b = [1, 2];
const arr_c = [-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];

console.log(uniqueOccurrences(arr_a));
console.log(uniqueOccurrences(arr_b));
console.log(uniqueOccurrences(arr_c));
