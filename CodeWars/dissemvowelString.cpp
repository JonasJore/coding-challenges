#include <iostream>
#include <string>
#define String std::string

bool isVowel(char c) {
  const char ch = toupper(c);
  return (ch == 'A' || ch == 'E' || ch == 'I' || ch == 'O' || ch == 'U');
}

String disemvowel(String str) {
  String finalStr = "";
  for(int i = 0; i < str.length(); i++) {
    if(!isVowel(str[i])) {
      finalStr += str[i];
    }
  }
  return finalStr;
}

int main() {
  std::cout << disemvowel("This website is for losers LOL!") << "\n";
}