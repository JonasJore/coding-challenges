fun sumConsecutives(s: List<Int>): List<Int> {
    val result = mutableListOf<List<Int>>()
    if (s.isNotEmpty()) {
        var current = mutableListOf(s.first())
        for (i in 1 until s.size) {
            if (s[i] == s[i - 1]) {
                current.add(s[i])
            } else {
                result.add(current)
                current = mutableListOf(s[i])
            }
        }
        result.add(current)
    }
    return result.map { it.sum() }
}