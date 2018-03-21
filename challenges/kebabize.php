<?php
function removeDigitsFromString($string) {
  return preg_replace('#[0-9 ]*#','',$string);
}

function kebabize($string) {
  $string = removeDigitsFromString($string);
  $charArr = str_split($string);
  $i = 0;
  while($i != count($charArr)) {
    if(ctype_upper($charArr[$i])) {
      $charArr[$i] = strtolower($charArr[$i]);
      $charArr[$i] = substr_replace($charArr[$i],'-',$charArr[$i],0);
    } 
    ++$i;
  }
  $charArr = implode($charArr);
  return ltrim($charArr,'-');
}