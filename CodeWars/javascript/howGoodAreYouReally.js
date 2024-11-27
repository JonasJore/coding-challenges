// link: https://www.codewars.com/kata/5601409514fc93442500010b/train/javascript

function betterThanAverage(classPoints, yourPoints) {
    if (!Array.isArray(classPoints)) throw Error("not array");
    return [...classPoints, yourPoints].reduce((prev, curr) => prev + curr, 0) / classPoints.length <= yourPoints;
}
