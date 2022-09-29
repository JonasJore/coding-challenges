// should turn letters that appear only once in the word should be encoded as "("
// all other characters that appear more than once should then be encoded as ")"
const duplicateEncode = (word) => 
  word.toLowerCase().split('')
    .map((val, index) => word.split('')
      .filter((val, index) => word.indexOf(val) != index).includes(val) ? ")" : "(").join('')

console.log(duplicateEncode("recede") === "()()()")
console.log(duplicateEncode("din") === "(((")
console.log(duplicateEncode("true") === "((((")