#include <iostream>
#include <string>
#include <map>

int solution(std::string roman) {
  std::map<char, int> romanChars {
    {'I', 1},{'V', 5},{'X', 10},{'L', 50},{'C', 100},{'D', 500},{'M', 1000}
  };
  int sum = 0;
  for(int i = 0; i < roman.length(); i++) {
    if(i+1 < roman.length()) {
      if(romanChars[roman[i]] >= romanChars[roman[i+1]]) {
        sum += romanChars[roman[i]];
      } else {
        sum += romanChars[roman[i+1]] - romanChars[roman[i]];
        ++i;
      }
    } else {
      sum += romanChars[roman[i]];
      ++i;
    }
  }
  return sum;
}

int main() {
  std::cout << solution("XXXXII");
}