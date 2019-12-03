from readInputFile import readInput

fileContent = readInput('./input/input_011219_03.txt')

opCodes = fileContent[0].split(',')

for i in range(0, len(opCodes)):
  opCodes[i] = int(opCodes[i])

def calculateOpCodes(opCodes):
  for i in range(0, len(opCodes) - 1, 4):
    if (opCodes[i] == 1):
      opCodes[opCodes[i + 3]] = opCodes[opCodes[i + 1]] + opCodes[opCodes[i + 2]]
    elif (opCodes[i] == 2):
      opCodes[opCodes[i+3]] = opCodes[opCodes[i + 1]] * opCodes[opCodes[i + 2]] 
    elif (opCodes[i] == 99):
      break
  
  return opCodes

print(calculateOpCodes(opCodes))