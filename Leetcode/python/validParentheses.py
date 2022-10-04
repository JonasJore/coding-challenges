class Solution(object):
    def isValid(self, s):
        """
        :type s: str
        :rtype: bool
        """
        stack = []
        for i in range(len(s)):
            if s[i] == "(" or s[i] == "[" or s[i] == "{":
                stack.append(s[i])
            else:
                if not stack:
                    return False
                
                lastChar = stack[len(stack)-1]

                if (lastChar == "(" and s[i] == ")") or (lastChar == "[" and s[i] == "]") or (lastChar == "{" and s[i] == "}"):
                    stack.pop()
                else:
                    return False

        if not stack:
            return True
        else:
            return False