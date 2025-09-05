package problems

fun problem1(): Int {
    print("Problem 1: ")
    val s = (1..999).filter {
        (it % 3 == 0 || it % 5 == 0)
    }
    return s.sum()
}
