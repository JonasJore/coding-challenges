# https://www.codewars.com/kata/5412509bd436bd33920011bc/python
def maskify(cc):
  arr = []
  if len(cc) <= 4:
    return cc
  for item in range(0, len(cc)):
    if item >= (len(cc) - 4):
      arr.append(cc[item])
    else:
      arr.append("#")
  return ''.join(arr)
