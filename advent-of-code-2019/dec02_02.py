from readInputFile import readInput

# under construction

correct = False

while not correct:
  for i in range(0,100):
    for j in range(0,100):
      fileContent = readInput('./input/input_011219_03.txt')

      opCodes = fileContent[0].split(',')

      for i in range(0, len(opCodes)):
        opCodes[i] = int(opCodes[i])

      opCodes[1] = i
      opCodes[2] = j

      for i in range(0, len(opCodes)-1, 4):
        if (opCodes[i] == 1):
          opCodes[opCodes[i + 3]] = opCodes[opCodes[i + 1]] + opCodes[opCodes[i + 2]]
        elif (opCodes[i] == 2):
          opCodes[opCodes[i + 3]] = opCodes[opCodes[i + 1]] * opCodes[opCodes[i + 2]] 
        elif (opCodes[i] == 99):
          break

      print(opCodes[0])

      if(opCodes[0] == 19690720):
        correct = True
        print("to obtain the output of 19690720 are " + str(i) + " and " + str(j))