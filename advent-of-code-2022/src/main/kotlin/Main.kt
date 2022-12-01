fun main() {
    printAnswers(
        "Day 1 part 1:",
        Day01().part1(),
        "Day 1 part 2:",
        Day01().part2()
    )
}

private fun <T> printAnswers(vararg args: T) {
    args.forEach { println(it) }
}
