import util.readInputToArr

internal class Day01 {
    private var input: List<String> = readInputToArr("day01_input.txt")

    fun part1() {
        var maxCalories = 0
        val food = mutableListOf<Int>()
        for (line in input) {
            if (line == "") {
                if (food.sum() > maxCalories) {
                    maxCalories = food.sum()
                }
                food.clear()
            } else {
                food.add(line.toInt())
            }
        }
        println(maxCalories)
    }

    fun part2() {
        val maxCalories = mutableListOf<Int>()
        val food = mutableListOf<Int>()

        for (line in input) {
            if (line == "") {
                maxCalories.add(food.sum())
                food.clear()
            } else {
                food.add(line.toInt())
            }
        }
        println(maxCalories.sortedDescending().take(3).sum())
    }
}
