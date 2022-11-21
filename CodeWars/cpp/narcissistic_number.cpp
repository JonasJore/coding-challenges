// https://www.codewars.com/kata/5287e858c6b5a9678200083c/cpp

#include <iostream>
#include <algorithm>
#include <vector>
#include <cmath>

std::vector<int> digits(int number) {
  std::vector<int> digitVector = {};
  while(number > 0) {
    int digit = number % 10;
    number /= 10;
    digitVector.push_back(digit);
  }
  std::reverse(digitVector.begin(), digitVector.end()); //TODO: could store digits in a stack instead to skip reversing step..
  // for (int i: digitVector)
  //   std::cout << i << std::endl;
  return digitVector;
}

bool narcissistic(int value){
  std::vector<int> vec = digits(value);
  int power = vec.size();
  int numbers = 0;
  for (int d: vec) {
    numbers += pow(d, power);
  }
  std::cout << numbers << " : final number!" << std::endl;
  return numbers == value;
}

int main() {
  std::cout << narcissistic(1652);
  return 0;
}

void tests() {
  std::cout << narcissistic(153) == true << std::endl;
  std::cout << narcissistic(1652) == false << std::endl;
}