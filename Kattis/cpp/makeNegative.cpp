//Just a fast kata i solved to not loose touch with C++ too much. 
//it is summer vacation after all ;) 

#include <iostream>


int makeNegative(int num) {
  if(num < 0) {
    return num;
  }
  
  return num *= (-1);
}

int main() {
  if(makeNegative(5) == -5) {
    std::cout << makeNegative(5) + " is equal to -5. thus, the test is a success!" << std::endl;
  } else {
    std::cout << "test failed";
  }
}