#include <iostream>
#include <vector>

std::vector<char>autori(std::string theString) {
  std::vector<char> resultVect;
  resultVect.push_back(theString[0]);
  for(int n = 0; n <= theString.length(); n++) {
    if(theString[n] == '-'){
      resultVect.push_back(theString[n + 1]);
    }
  }
  return resultVect;
}


void printVector(std::vector<char> theVector) {
  for(char &c : theVector) {
    std::cout << c;
  }
}

int main() {
  std::string input;
  std::cin >> input;
  printVector(autori(input));
}