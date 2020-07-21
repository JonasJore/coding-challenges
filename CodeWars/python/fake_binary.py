# https://www.codewars.com/kata/57eae65a4321032ce000002d/python
def fake_bin(x):
  return ''.join(map(lambda i: '0' if int(i) < 5 else '1', x))

# tests:
print(fake_bin("45385593107843568") == "01011110001100111")
print(fake_bin("420") == "000")
print(fake_bin("1337") == "0001")
print(fake_bin("5550009333555") == "1110001000111")