#include <iostream>
#include <ctype.h>

//takes in a string and rotates all chars 13 positions. returns a rot13 encrypted string.
std::string rot13(std::string msg){
  int i = 0;
  while(i < msg.length()) {
    if(isalpha(msg[i])) {
      if(msg[i] >= 'a' && msg[i] <= 'm') msg[i] += 13;
      else if(msg[i] >= 'A' && msg[i] <= 'M') msg[i] += 13;
      else if(msg[i] >= 'n' && msg[i] <= 'z') msg[i] -= 13;
      else if(msg[i] >= 'N' && msg[i] <= 'Z') msg[i] -= 13;
    }
    ++i;
  }
  return msg;
}

int main() {
  //testing
  std::cout << rot13("test") << std::endl;
  std::cout << rot13("pingpong") << std::endl;
  std::cout << rot13("@1a2s3d1f2g3@") << std::endl;
}