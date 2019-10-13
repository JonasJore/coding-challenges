#https://www.codewars.com/kata/52774a314c2333f0a7000688/python
#validates if parentheses checks out correctly

def valid_parentheses(string):
  numPar = 0

  for i in range(0, len(string)):

    if string[i] == '(': numPar += 1
    if string[i] == ')': numPar -= 1
    if numPar < 0: return False
    
  return numPar == 0

print(valid_parentheses("()()()()((((((((()))))))))"))