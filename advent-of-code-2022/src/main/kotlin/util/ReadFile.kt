package util

import java.io.File

internal fun readInputToArr(fileName: String): List<String> {
    return File("src/input", fileName).readLines()
}
