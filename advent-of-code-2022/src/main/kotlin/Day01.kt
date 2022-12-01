import util.readInputToArr

internal class Day01 {
    private var input: List<String> = readInputToArr("day01_input.txt")

    fun part1(): Int {
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
        return maxCalories
    }

    fun part2(): Int {
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
        return maxCalories.sortedDescending().take(3).sum()
    }
}
