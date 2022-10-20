// https://www.codewars.com/kata/526571aae218b8ee490006f4

function decimalToBinary(n: number): string {
    return n.toString(2);
}

export function countBits(n: number): number {
    return decimalToBinary(n).split("").filter((value, _) => value === "1").length;
}

console.log(countBits(0) === 0);
console.log(countBits(4) === 1);
console.log(countBits(7) === 3);
console.log(countBits(9) === 2);
console.log(countBits(10) === 2);
console.log(countBits(504514958749753) === 25);
