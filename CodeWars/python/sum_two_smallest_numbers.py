# TODO: return sum of the two smallest values in the list
def sum_two_smallest_numbers(numbers):
  first = float('inf')
  second = float('inf') 

  for number in numbers:
    if number < first:
      second = first
      first = number
    elif number < second:
      second = number
  return first + second

def sum_two_smallest_numbers_second_approach(numbers):
  sortedList = sorted(numbers)
  return sortedList[0] + sortedList[1]

#testing:
print(sum_two_smallest_numbers_second_approach([10, 343445353, 3453445, 3453545353453]))
