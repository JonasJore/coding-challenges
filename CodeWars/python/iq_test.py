#https://www.codewars.com/kata/552c028c030765286c00007d

def iq_test(numbers):
  evens, odds = [],[]
  numberList = numbers.split(' ')

  for i in range(0, len(numberList)):
    intNumber = int(numberList[i])
    if intNumber % 2 == 0:
      evens.append([intNumber, i + 1])
    else:
      odds.append([intNumber, i + 1])

  return odds[0][1] if len(odds) == 1 else evens[0][1]   

print(iq_test("1 2 2"))
print(iq_test("18 24 40 2 264 99 1000"))
