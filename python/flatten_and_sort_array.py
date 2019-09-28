# takes in a 2d Array, flattens and sorts it

def flatten_and_sort(array):
    flattenedArr = []

    for subArr in array:
      for item in subArr:
        flattenedArr.append(item)

    return sorted(flattenedArr)
