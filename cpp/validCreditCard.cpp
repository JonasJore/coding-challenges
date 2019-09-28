#include <iostream>
#include <vector>

class Kata {
  public:
  static bool validate(long long int n);
};

std::vector<long long> numberToDigits(long long int number) {
  std::vector<long long> digits;
  while(number > 0) {
    int digit = number % 10;
    number /= 10;
    digits.insert(digits.begin(), digit);
  }
  return digits;
}

bool Kata::validate(long long n) {
  std::vector<long long> digits = numberToDigits(n);
  if(digits.size() % 2 == 0) {
    for(int i = 0; i < digits.size(); i+=2) {
      digits[i] += digits[i];
    }  
  } else {
    for(int i = 1; i < digits.size(); i+=2) {
      digits[i] += digits[i];
    }
  }

  int sum = 0;
  for(int i = 0; i < digits.size(); i++) {
    if(digits[i] > 9) {
      digits[i] -= 9;
    }
    sum += digits[i];
  }
  
  return sum % 10 == 0;
}


int main() {
  std::cout << Kata::validate(2121);
}