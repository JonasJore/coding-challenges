#include <iostream>
#include <string>
#include <vector>
#include <sstream>

bool isVowel(char c) {
  if(c >= 'A' && c <= 'Z') {
    return (c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U');
  }
  return (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u');
}

std::vector<std::string> splitSentence(std::string str) {
  std::vector<std::string> words;
  std::string word = "";
  for(char character : str) {
    if(character == ' ') {
      words.push_back(word);
      word = "";
    } else {
      word = word + character;
    }
  }
  return words;
}

std::string pigLatinConverter(std::string str) {
  int vowelPosition = 999;
  for(int i = 0; i < str.length(); i++) {
    if(isVowel(str[i])) {
      vowelPosition = i;
      break;
    }
  }
  if(vowelPosition == 999) {
    return "no vowels...";
  }
  std::string piglatinWord = str.substr(vowelPosition) + str.substr(0, vowelPosition) + "ay";
  return piglatinWord;
}

std::string pig_it(std::string str) {
  std::vector<std::string> words = splitSentence(str);
  std::string piglatinSentence = "";
  if(words.size() > 1) {
    for(std::string word : words) {
      piglatinSentence += pigLatinConverter(word) + " ";
    }
  } else {
    piglatinSentence = pigLatinConverter(str);
  }
  return piglatinSentence;
}

int main() {
  std::cout << pig_it("Hello world !");
}