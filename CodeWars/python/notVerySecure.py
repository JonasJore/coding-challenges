# https://www.codewars.com/kata/526dbd6c8c0eb53254000110/train/

import re

def alphanumeric(password):
  if re.match(r"^[0-9a-zA-Z]+$", password):
    return True
  return False

print(alphanumeric("j0naS") == True)
print(alphanumeric("@&owaSP") == False)
