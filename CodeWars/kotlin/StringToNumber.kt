// https://www.codewars.com/kata/544675c6f971f7399a000e79/kotlin

class Solution {

  fun stringToNumber(str: String): Int {
    return if (str.isNotBlank() && str.toIntOrNull() != null) {
      str.toInt()
    } else 0
  }
}

class Testing {

  private val solution = Solution()

  init {
    basicTests()
  }

  private fun basicTests() {
    println(1234 == solution.stringToNumber("1234"))
    println(-69 == solution.stringToNumber("-69"))
    println(0 == solution.stringToNumber("not a number"))
  }
}

fun main() {
  Testing()
}