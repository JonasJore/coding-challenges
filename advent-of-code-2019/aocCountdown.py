from dateutil.relativedelta import relativedelta
import datetime

def daysUntilAdventOfCode_2019():
  today = datetime.date.today()
  firstOfDecember = datetime.date(2019, 12, 1)
  difference = relativedelta(today, firstOfDecember)

  return difference.__dict__['days'] * (-1)

print(daysUntilAdventOfCode_2019())
