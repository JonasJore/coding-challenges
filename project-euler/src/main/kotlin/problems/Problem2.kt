package problems

fun fibonacciSequence() = sequence {
    var a = 1
    var b = 2
    yield(a)
    yield(b)
    while (true) {
        val next = a + b
        yield(next)
        a = b
        b = next
    }
}

fun problem2() {
    print("Problem 2: ")
    val seq = fibonacciSequence().takeWhile { it <= 4_000_000 }.toList()
    val res = seq.filter { it % 2 == 0 }

    print(res.sum())
}
