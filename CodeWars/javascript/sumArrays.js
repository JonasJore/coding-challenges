// Sum Numbers
function sum(numbers) {
    "use strict";
    const sum = numbers.reduce((acc, curr) => acc + curr, 0);
    return sum;
}

function main() {
    const s = sum([1, 2, 3, 4]);
    console.log(s);
}

main();
