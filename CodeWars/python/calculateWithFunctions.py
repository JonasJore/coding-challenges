# https://www.codewars.com/kata/525f3eda17c7cd9f9e000b39/python

def zero(f = None): return 0 if not f else f(0)
def one(f = None): return 1 if not f else f(1)
def two(f = None): return 2 if not f else f(2)
def three(f = None): return 3 if not f else f(3)
def four(f = None): return 4 if not f else f(4)
def five(f = None): return 5 if not f else f(5)
def six(f = None): return 6 if not f else f(6)
def seven(f = None): return 7 if not f else f(7)
def eight(f = None): return 8 if not f else f(8)
def nine(f = None): return 9 if not f else f(9)

def plus(y): return lambda x: x + y
def minus(y): return lambda x: x - y
def times(y): return lambda x: x * y
def divided_by(y): return lambda x: x // y

# testing: 
print(two(times(seven()))) # returns 14
print(eight(divided_by(two()))) # returns 4
print(nine(divided_by(three()))) # returns 3
print(seven(times(seven()))) # returns 49
print(nine(times(nine()))) # returns 81
