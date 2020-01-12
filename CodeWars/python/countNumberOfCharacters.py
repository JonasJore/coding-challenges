# https://www.codewars.com/kata/52efefcbcdf57161d4000091

def count(string):
  dict = {}
  for letter in string:
    if letter in dict:
      dict[letter] += 1
    else:
      dict[letter] = 1
  return dict
