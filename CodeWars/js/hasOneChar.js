// https://www.codewars.com/kata/584fa5ae25dd087e6b000070
const hasOneChar = (s) => new Set(s.split('')).size === 1 ? true : false

// tests:
console.log(hasOneChar("jjjjjjjjjj") === true)
console.log(hasOneChar("aaaaaaaaaabaaaaaaaaaaa") === false)
console.log(hasOneChar("0000000000000000000000") === true)
