#include <cmath>

bool PythagoreanTriple(const int a, const int b, const int c) {
    return (pow(a,2) + pow(b,2) == pow(c,2)) ? true : false;
}

#include <iostream>

int main() {
    std::cout << PythagoreanTriple(3,4,5) << std::endl; //will return true (1).
    std::cout << PythagoreanTriple(4,2,1) << std::endl; //will return false (0).
}