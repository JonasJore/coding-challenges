#include <iostream>

std::string hiss_detector(std::string s){
  for(int i = 0; i < s.length(); i++){
    if(s[i] == 's' && s[i] == s[i+1]){
      return "hiss";
    }
  }
  return "no hiss";
}

int main(){
  std::string input;
  std::cin >> input;
  std::cout << hiss_detector(input) << std::endl;
  return 0;
}