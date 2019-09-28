//returns the sum of multiples of 3 and 5 up until given number
function solution(number){
  var sum = 0;
  for(var i = 0; i < number; i++) {
    if(i % 3 === 0 || i % 5 === 0) {
      sum += i;
    }
  }
  return sum;
}