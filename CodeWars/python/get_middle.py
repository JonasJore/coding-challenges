#returns middle char of given string. if len is even return
#the two center chars

def get_middle(s):
    if len(s) % 2 != 0:
      return s[len(s) / 2]
    else:
      return s[len(s) / 2 - 1] + s[len(s) / 2] 

#testing:
print(get_middle("Aardvark"))
print(get_middle("Pacman"))
print(get_middle("J_oJ_"))