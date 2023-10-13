// https://www.codewars.com/kata/5782dd86202c0e43410001f6/javascript

function doMath(string) {
  const letters = 'abcdefghijklmnopqrstuvwxyz';

  const list = string.split(' ')
    .sort((a, b) => {
      const letterA = a.match(/[a-z]/i);
      const letterB = b.match(/[a-z]/i);

      return letterA[0].localeCompare(letterB[0]);
    })
    .map((value) => value.split('')
      .filter(char => !letters.includes(char)).join(''))
    .map((item) => Number(item));

  let result = list[0];
  const operations = ['+', '-', '*', '/'];
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

  tests();
}

function tests() {
  let testcases = [
    doMath("24z6 1z23 y369 89z 900b") == 1414,
    doMath("24z6 1x23 y369 89a 900b") == 1299,
    doMath("10a 90x 14b 78u 45a 7b 34y") == 60,
    doMath("111a 222c 444y 777u 999a 888p") == 1459,
    doMath("1z 2t 3q 5x 6u 8a 7b") == 8
  ]

  testcases.forEach((value) => {
    if (value === false) {
      throw Error("test case failed");
    }
  });
}

main();