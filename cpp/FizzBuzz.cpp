/**
 *  Simple challenge
 *  prints `Fizz` if i is divisible with x, `Buzz` if i is divisible with y,
 *  and `FizzBuzz` i i is both divisible with x and y.
 *  
 *  Date: 01/02/18
*/
#include <iostream>

void fizzBuzzChecker(int x, int y, int n){
  for(int i = 1; i <= n; i++){
    if(((i % x) || (i % y)) == 0){
      std::cout << "FizzBuzz" << std::endl;
    } else if((i % x) == 0){
      std::cout << "Fizz" << std::endl;
    } else if((i % y) == 0){
      std::cout << "Buzz" << std::endl;
    } else{
      std::cout << i << std::endl;
    }
  }
}


int main(){
  int x,y,n;
  std::cin>>x>>y>>n;
  fizzbuzzChecker(x,y,n);
  return 0;
}