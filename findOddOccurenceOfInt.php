<?php 
function findIntThatOccuresAnOddAmountOfTimes(array $seq) : int {
  for($i = 0; $i < count($seq); $i++) {
    $counter = 0;
    for($j = 0; $j < count($seq); $j++) {
      if($seq[$i] == $seq[$j]) {
        $counter++;
      }
    }
    if($counter % 2 != 0) {
      return $seq[$i];
    }
  }
  return -1;
}

function testerFunction() {
  echo 'Test 1: '.findIntThatOccuresAnOddAmountOfTimes([2,5,3,2,5,7,4,6,3,5,3,2,6,4,3,6,3,6,2,6,9,7,5])."\n";
  echo 'Test 2: '.findIntThatOccuresAnOddAmountOfTimes([-1,-1,3,2,4,22,55,33,1,-20,330,201,1,'3',2,4,22])."\n";
  
}

testerFunction();
?>