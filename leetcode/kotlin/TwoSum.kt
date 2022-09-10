class TwoSum {

  fun twoSum(nums: IntArray, target: Int): IntArray {
    val hashMap = HashMap<Int, Int>()

    nums.forEachIndexed { index, item ->
      val indexOfFound = hashMap[target - item]
      indexOfFound?.let {
        return intArrayOf(it, index)
      }
      hashMap[item] = index
    }

    throw IllegalArgumentException("Impossible")
  }
}
