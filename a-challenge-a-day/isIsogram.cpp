#include <iostream>
#include <map>
#define Map std::map

bool is_isogram(std::string str) {
  Map<char, bool> dict;
  if(str.length() == 1) {
    return true;
  }
  for(auto c : str) {
    if(dict[tolower(c)] == true) {
      return false;
    } else {
      dict[tolower(c)] = true;
    }
  }
  return true;
}