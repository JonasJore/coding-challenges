//source: https://www.codewars.com/kata/57356c55867b9b7a60000bd7/c
int basic_op(char op, int value1, int value2) {
  switch (op) {
      case '+': return value1 + value2;
      case '-': return value1 - value2;
      case '*': return value1 * value2;
      case '/': return value1 / value2;
      default:
          return value1;
  }
  return 0;
}
