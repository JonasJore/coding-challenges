// source: https://www.codewars.com/kata/515f51d438015969f7000013/cpp

#include <iostream>
#include <vector>
#include <cstdlib>

std::vector<std::vector<int>> pyramid(std::size_t n) {
  std::vector<std::vector<int>> parentVector;
  std::vector<int> childVector;
  for(int i = 1; i <= n; i++) {
    for(int j = 0; j <= i; j++) {
      childVector.push_back(1);
    }
    parentVector.push_back(childVector);
  }

  return parentVector;
}

void testPyramid(std::size_t size) {
  std::cout << "[ ";
  std::vector<std::vector<int>> task = pyramid(size);
  for(int i = 0; i < task.size(); i++) {
    std::cout << " [ ";
    for(int j = 0; j <= i; j++) {
      std::cout << task[i][j];
    }
    std::cout << " ] ";
  }
  std::cout << " ]" << std::endl;

}

int main() {
  testPyramid(1); // [ [ 1 ] ]
  testPyramid(2); // [ [ 1 ], [ 1,1 ] ]
  testPyramid(3); // [ [ 1 ], [ 1,1 ], [ 1,1,1 ] ]
  testPyramid(4); // [ [ 1 ], [ 1,1 ], [ 1,1,1 ] [ 1,1,1,1 ] ]
}
