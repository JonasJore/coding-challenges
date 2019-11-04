#source: https://www.codewars.com/kata/59706036f6e5d1e22d000016/python

def createScoreDict():
  letters = "0abcdefghijklmnopqrstuvwxyz"
  obj = {}

  for i in range(0, len(letters)):
    obj[letters[i]] = i

  return obj

def words_to_marks(s):
  letters = createScoreDict()
  score = 0

  for i in range(0, len(s)):
    score += letters[s[i]]

  return score

#testing:
print(words_to_marks("friendship") == 108)
print(words_to_marks("jonas") == 59)
