#https://www.codewars.com/kata/537baa6f8f4b300b5900106c/python
import math

def circle_area(r):
    if type(r) == str:
        return False
    if r < 1:
        return False
    area = round(math.pi * (round(r, 2)**2), 2)
    return area