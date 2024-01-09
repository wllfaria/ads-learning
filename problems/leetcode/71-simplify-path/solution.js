/**
 * @param {string} path
 * @return {string}
 */
var simplifyPath = function (path) {
    const canonical = [];
    path = path.split("/").filter((s) => !!s);

    while (path.length != 0) {
        const curr = path.shift();
        if (curr === ".") continue;
        else if (curr === "..") canonical.pop();
        else canonical.push(curr);
    }
    return "/" + canonical.join("/");
};

const path_a = "/home/";
const path_b = "/../";
const path_c = "/home//foo/";
const path_d = "/a/./b/../../c/";

console.log(simplifyPath(path_a));
console.log(simplifyPath(path_b));
console.log(simplifyPath(path_c));
console.log(simplifyPath(path_d));
