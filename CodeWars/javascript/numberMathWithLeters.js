// https://www.codewars.com/kata/5782dd86202c0e43410001f6/javascript

function doMath(string) {
  const letters = 'abcdefghijklmnopqrstuvwxyz';

  const list = string.split(' ')
    .sort((a, b) => {
      const letterA = a.match(/[a-z]/i);
      const letterB = b.match(/[a-z]/i);

      return letterA[0].localeCompare(letterB[0]);
    })
    .map((value, _) => value.split('')
      .filter(char => !letters.includes(char)).join(''))
    .map((item, _) => Number(item));

  let result = list[0];
  const operations = ["+", "-", "*", "/"];
  let operationIndex = 0;
  let currentOperation = operations[operationIndex];

  for (let i = 1; i < list.length; i++) {
    if (operationIndex > 3) {
      operationIndex = 0;
    }

    currentOperation = operations[operationIndex]

    if (currentOperation === '+') {
      result += list[i];
    } else if (currentOperation === '-') {
      result -= list[i];
    } else if (currentOperation === '*') {
      result *= list[i];
    } else if (currentOperation === '/') {
      result /= list[i];
    }

    console.log(`iteration: ${i} `, {
      resultSoFar: result,
      result: list[i],
      operationIndex,
      currentOperation
    });

    operationIndex += 1;
  }

  return Math.round(result);
}

function main() {
  const res = doMath("1z 2t 3q 5x 6u 8a 7b");
  console.log(res);
}

main();