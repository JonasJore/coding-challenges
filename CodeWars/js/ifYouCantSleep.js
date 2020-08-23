// https://www.codewars.com/kata/5b077ebdaf15be5c7f000077
var countSheep = function (num) {
  let sheep = ""
  for (let i = 1; i <= num; i++) {
    sheep += `${i} sheep...`
  }

  return sheep
}

const countSheep2 = (num) => {
  return Array.from({ length: num }).map((_, i) => {
    return `${i + 1} sheep...`
  }).join("")
}

// actual answer
const countSheep3 = (num) =>
  Array.from({ length: num }).map((_, i) => `${i + 1} sheep...`).join("")

console.log(
  countSheep2(1),
  "1 sheep..."
)

console.log(
  countSheep(3),
  "1 sheep...2 sheep... 3 sheep..."
)