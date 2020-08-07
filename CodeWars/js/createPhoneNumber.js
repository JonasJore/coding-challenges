// https://www.codewars.com/kata/525f50e3b73515a6db000b83/javascript

function createPhoneNumber(numbers) {
  const formatted = numbers.join('').match(/^(\d{3})(\d{3})(\d{4})$/);
  return `(${formatted[1]}) ${formatted[2]}-${formatted[3]}`;
}

console.log(createPhoneNumber([1, 2, 3, 4, 5, 6, 7, 8, 9, 0])=="(123) 456-7890") // true
console.log(createPhoneNumber([1, 1, 1, 1, 1, 1, 1, 1, 1, 1])=="(111) 111-1111") // true