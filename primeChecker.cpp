/**
 * prints all the primenumbers given in a range of ints
 * Date: 06.02.18
 * 
 */

#include <iostream>
using namespace std;

int primeChecker(int n){
  if(n==1){
    return false;
  }
  int i = 2;
  while(i * i <= n){
    if(n % i == 0){
      return false;
    }
    i++;
  }
  
  return true;
}

int main(){
  int input, start, range;
  cin>>input;
  for(int i = 0; i < input; i++){
    cin>>start>>range;
    for(int j = start; j <= range; j++){
      if(primeChecker(j)==true){
        cout << j << endl;
      }
      
    }
    cout << endl;
  }
  return 0;
}