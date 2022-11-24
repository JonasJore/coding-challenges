// https://www.codewars.com/kata/517abf86da9663f1d2000003/kotlin

class Solution {

  private fun getDelimiter(str: String): String? {
    return when {
      str.contains("-") -> "-"
      str.contains("_") -> "_"
      else -> null
    }
  }

  fun toCamelCase(str: String): String {
    val delimiter = getDelimiter(str) ?: return str
    val wordList: List<String> = str.split(delimiter)

    if (wordList.size == 1 && wordList[0].isBlank())
      return str

    return if (wordList[0][0].isUpperCase())
      wordList.joinToString("") { item ->
        item.replace(item[0], item[0].uppercaseChar())

      }
    else
      wordList.mapIndexed { index, item ->
        if (index != 0)
          item.replace(item[0], item[0].uppercaseChar())
        else item
      }.joinToString("")
  }
}

class Testing {

  private val solution = Solution()

  init {
    tests()
  }

  private fun tests() {
    println(solution.toCamelCase("The_stealth_warrior"))
    println(solution.toCamelCase("gfgfh-stealth-warrior"))
    println(solution.toCamelCase(""))
    println(solution.toCamelCase("...ntoNDNYpTCCBCNwVSqNn[v]FDM"))
    println("theStealthWarrior" == solution.toCamelCase("the_stealth_warrior"))
    println("TheStealthWarrior" == solution.toCamelCase("The-Stealth-Warrior"))
    println("ABC" == solution.toCamelCase("A-B-C"))
    println("...ntoNDNYpTCCBCNwVSqNn[v]FDM" == solution.toCamelCase("...ntoNDNYpTCCBCNwVSqNn[v]FDM"))
    println("" == solution.toCamelCase(""))
  }
}

fun main() {
  Testing()
}