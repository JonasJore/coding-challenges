import math
from readInputFile import readInput

def fuelRequired(moduleMass):
  return math.floor(moduleMass / 3) - 2

inputList = readInput('./input/input_011219_01.txt')

sum = 0

for line in inputList:
  sum += fuelRequired(int(line))

print("Answer - December 1st - part 01: ", sum)
