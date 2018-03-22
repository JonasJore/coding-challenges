//Returns numbers that are the sum of its own digits raised to the consecutive power.
//within given range parameters.
//Awesome!

function sumDigPow(a, b) {
    var arr = [];
    for(var i = a; i <= b; i++) {
      i = i.toString();
      var sum = 0;
      var incrementor = 0;
      while(incrementor < i.length) {
        sum += Math.pow(parseInt(i[incrementor]),(incrementor + 1));
        incrementor++;
      } 
      if(sum === parseInt(i)) {
          arr.push(parseInt(i));
      }
    }
    return arr;
  }
  
  function testerFunction() {
    return sumDigPow(1,100);
    // returns [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 89 ]
  }
  
  
  testerFunction();