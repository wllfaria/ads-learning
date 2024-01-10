var rotate = function (matrix) {
    const m = [];
    for (let i = 0; i < matrix.length; ++i) {
        m.push([]);
    }

    for (let c = 0; c < matrix.length; ++c) {
        for (let l = matrix.length - 1; l >= 0; --l) {
            m[c].push(matrix[l][c]);
        }
    }

    for (let i = 0; i < m.length; ++i) {
        matrix[i] = m[i];
    }
};

const matrix_a = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
];

const matrix_b = [
    [5, 1, 9, 11],
    [2, 4, 8, 10],
    [13, 3, 6, 7],
    [15, 14, 12, 16],
];

rotate(matrix_a);
console.log(matrix_a);
rotate(matrix_b);
console.log(matrix_b);
