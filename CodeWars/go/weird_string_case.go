// https://www.codewars.com/kata/52b757663a95b11b3d00062d/go

package main

import (
    "fmt"
    "strings"
)

func isEven(index int) bool {
    return index % 2 == 0
}

func toWeirdCase(str string) string {
    words := strings.Split(str, " ")
    var weirdArr []string
    for _, word := range words {
        chars := strings.Split(word, "")
        for index, char := range chars {
            if isEven(index) {
                weirdArr = append(weirdArr, strings.ToUpper(char))
            } else {
                weirdArr = append(weirdArr, char)
            }
        }
        weirdArr = append(weirdArr, " ")
    }
    return strings.Join(weirdArr, "")
}

func main() {
    var weird = toWeirdCase("abc def")
    var weird2 = toWeirdCase("String")
    var weird3 = toWeirdCase("ABC")
    var weird4 = toWeirdCase("This is a test Looks like you passed")
    fmt.Println(weird)
    fmt.Println(weird2)
    fmt.Println(weird3)
    fmt.Println(weird4)
}
