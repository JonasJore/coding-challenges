import math
from readInputFile import readInput
from dec01_01 import fuelRequired

fileLine = readInput('./input/dec_01-part-1.txt')

sum = 0
for module in fileLine:
  fuel = fuelRequired(int(module))
  fuelTotal = fuel
  while(fuel > 0):
    fuel = fuelRequired(fuel)
    if fuel > 0:
      fuelTotal += fuel
  sum += fuelTotal

print("Answer - December 1st - part 02: ", sum)
