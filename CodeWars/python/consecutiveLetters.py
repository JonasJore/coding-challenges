#https://www.codewars.com/kata/5ce6728c939bf80029988b57/python

def solve(st):
  if(len(st) == 1):
    return True
    
  sortedInput = ''.join(sorted(st))

  for i in range(1, len(st)):
    if ord(sortedInput[i]) - ord(sortedInput[i - 1]) != 1:
      return False
  return True 

#Tests:
print(solve("abc"))
print(solve("abd"))
print(solve("dabc"))
print(solve("abbc"))
print(solve("v"))