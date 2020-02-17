# https://www.codewars.com/kata/5a2be17aee1aaefe2a000151

def getSum(i):
  return 0 if len(i) == 0 else i[0] + getSum(i[1:])

def array_plus_array(arr1,arr2):
    return getSum(arr1) + getSum(arr2)
