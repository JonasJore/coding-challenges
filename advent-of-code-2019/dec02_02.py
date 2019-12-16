from readInputFile import readInput

def getInput():
  input = readInput('./input/dec_02-part-2.txt')[0].split(',')
  return list(map(int, input))

def opcodeComputer(op):
  for i in range(0, len(op) - 1, 4):
    if (op[i] == 1):
      op[op[i + 3]] = op[op[i + 1]] + op[op[i + 2]]
    elif (op[i] == 2):
      op[op[i + 3]] = op[op[i + 1]] * op[op[i + 2]]
    elif (op[i] == 99):
      return op[0]
      break
  return None

def findNumberOfNounsAndVerbs():
  for noun in range(0,100):
    for verb in range(0,100):
      input = getInput()

      input[1] = noun
      input[2] = verb
      result = opcodeComputer(input)

      print(result)

      if result == 19690720:
        return 100 * noun + verb

  print("nothing found")


findNumberOfNounsAndVerbs()
