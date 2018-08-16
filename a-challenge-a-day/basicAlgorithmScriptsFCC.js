function convertToF(celsius) {
  let fahrenheit
  return fahrenheit = celsius * (9/5) + 32
}

function reverseString(str) {
  return str.split('').reverse().join('')
}

function factorialize(num) {
  let result = 1
  for(let i = 1; i <= num; i++) {
    result *= i
  }
  return result
}

function findLongestWordLength(str) {
  let longestWord = 0
  let words = str.split(' ')
  for(let i = 0; i < words.length; i++) {
    if(words[i].length > longestWord){
      longestWord = words[i].length
    }
  }
  return longestWord
}

function largestOfFour(arr) {
  let results = []
  for(let i = 0; i < arr.length; i++) {
    let largestNum = arr[i][0]
    for(let j = 1; j < arr[i].length; j++) {
      if(arr[i][j] > largestNum) {
        largestNum = arr[i][j]
      }
    }
    results[i] = largestNum
  }
  return results
}

function confirmEnding(str, target) {
  return str.substr(-target.length) === target;
}

function repeatStringNumTimes(str, num) {
  let repeatedString = ''
  while(num > 0) {
    repeatedString += str
    --num
  }

  return repeatedString
}
