import util.readInputToArr

internal class Day03 {
    private val input: List<String> = readInputToArr("day03_input.txt")
    private val priorities = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

    private fun commonElements(pair: Pair<String, String>): String {
        val common = mutableSetOf<String>()
        pair.first.forEach { item ->
            if (pair.second.contains(item))
                common.add(item.toString())
        }

        return common.joinToString("")
    }

    fun part1(): String {
        val rucksacksWithCompartments =
            input.map {
                commonElements(
                    Pair(
                        it.substring(0, it.length / 2),
                        it.substring(it.length / 2, it.length)
                    )
                )
            }

        val priorityPerEntry: List<Int> = rucksacksWithCompartments.map {
            it.sumOf { item ->
                this.priorities.indexOf(item)
            }
        }

        return priorityPerEntry.sum().toString()
    }

    fun part2(): String {
        val priorityOfItemPerGroup = input
            .chunked(3)
            .map { item ->
                val sorted = item.sortedBy { it.length }
                val common = mutableSetOf<String>()

                sorted.first().forEach {
                    if (sorted[1].contains(it) && sorted[2].contains(it))
                        common.add(it.toString())
                }
                common.toList()
            }.map {
                it.sumOf { item ->
                    this.priorities.indexOf(item)
                }
            }

        return priorityOfItemPerGroup.sum().toString()
    }
}
