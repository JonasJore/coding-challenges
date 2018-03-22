/**
 *  Another simple challenge
 *  Mods all numbers in an int array with `42` and counts how many unique numbers there are.
 * 
 *  Date: 01.02.18
 * 
 */

#include <iostream>
#include <unordered_set>

int countUniques(std::unordered_set<int> hash_table, int mod_num[], size_t size){
  return hash_table.size();
}

int main(){
  int unique = 0;
  const size_t size = 10;
  int numbers[size];
  int mod_num[size];
  
  for(int i = 0; i < 10; i++){
    std::cin>>numbers[i];
    mod_num[i] = numbers[i] % 42;
  }

  std::unordered_set<int> hash_table(mod_num, mod_num + size);
  std::cout << countUniques(hash_table, mod_num, size) << std::endl;
  
  return 0;
}