#include <iostream>
using namespace std;

size_t ahCounter(string ah){
  size_t count = 0;
  for(char a : ah){
    if(a == 'a'){
      ++count;
    }
  }
  return count;
}

string doctors_Opinion(string ah, string aah){
  if(ahCounter(ah) >= ahCounter(aah)){
    return "go";
  } else if(ahCounter(ah) <= ahCounter(aah)){
    return "no";
  }
}

int main() {
  string ah, aah;
  cin >> ah >> aah;
  cout << doctors_Opinion(ah,aah) << endl;
  
  return 0;
}