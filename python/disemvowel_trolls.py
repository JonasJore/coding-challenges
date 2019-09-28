def disemvowel(string):
    disemvoweledString = ''

    for i in string:
      if i not in 'aeiouAEIOU':
        disemvoweledString += i

    return disemvoweledString

# testing:
print(disemvowel("this python script sucks, get outta here"))
