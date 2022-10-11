class Solution {

  fun splitString(str: String): List<String> {
    val chunks = str.windowed(2, step = 2, partialWindows = true).map {
      if (it.length != 2) {
        "${it}_"
      } else {
        it
      }

    }
    return chunks
  }
}

fun main(args: Array<String>) {
  val s = Solution().splitString("abcdef")
  println(s)
}
