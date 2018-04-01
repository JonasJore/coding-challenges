#include <iostream>
#include <vector>

std::vector<std::string> buildTower(int floors) {
  std::vector<std::string> arr;
  std::string spacing = "";
  for(int i = floors; i > 0; i--) {
    std::string star = "*";
    for(int j = i-1; j > 0; j--) {
      star += "**";
    }
    star = spacing + star + spacing;
    arr.push_back(star);
    spacing += " ";
  }
  return arr;
}

void printTower(std::vector<std::string> towerVector) {
  for(int i = towerVector.size()-1; i > 0; i--) {
    std::cout << towerVector[i] << std::endl;
  }
}

int main() {
  printTower(buildTower(6));
}