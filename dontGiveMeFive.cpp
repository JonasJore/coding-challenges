#include <iostream>

bool containsFive(std::string s) {
  for(int i = 0; i <= s.length(); i++) {
    if(s[i] == '5') {
      return true;
    }
  }
  return false;
}

int dontGiveMeFive(int start, int end) {
  int count = 0;
  for(int i = start; i <= end; i++) {
    std::string convertedToString = std::to_string(i);
    if(i == 5 || containsFive(convertedToString)) {
      count -= 1;
    } 
    ++count;
  }
  return count;
}

int main() {
  std::cout << dontGiveMeFive(1,16) << std::endl;
}