#include <iostream>

void fizzbuzzChecker(int x, int y, int n){
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