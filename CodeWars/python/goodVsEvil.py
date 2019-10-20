# source: https://www.codewars.com/kata/52761ee4cffbc69732000738/python

GOOD_WIN = 'Battle Result: Good triumphs over Evil'
EVIL_WIN = 'Battle Result: Evil eradicates all trace of Good'
DRAW = 'Battle Result: No victor on this battle field'

def worthOfArmy(army, faction):
  armyList = list(map(int, army.split()))
  if faction == 'good':
    return sum([armyList[0]*1,armyList[1]*2,armyList[2]*3,armyList[3]*3,armyList[4]*4,armyList[5]*10])
  if faction == 'evil':
    return sum([armyList[0]*1,armyList[1]*2,armyList[2]*2,armyList[3]*2,armyList[4]*3,armyList[5]*5,armyList[6]*10])
  
def goodVsEvil(good, evil):
  goodSum,evilSum = worthOfArmy(good, 'good'), worthOfArmy(evil, 'evil')
  if goodSum > evilSum:
    return GOOD_WIN
  if goodSum < evilSum:
    return EVIL_WIN
  
  return DRAW
