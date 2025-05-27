def backward_string(val: str) -> str:
    result = ""
    for i in range(len(val),0,-1):
        result += val[i-1]
    return result


print("Example:")
print(backward_string("val"))

# These "asserts" are used for self-checking
assert backward_string("val") == "lav"
assert backward_string("") == ""
assert backward_string("ohho") == "ohho"
assert backward_string("123456789") == "987654321"

print("The mission is done! Click 'Check Solution' to earn rewards!")

