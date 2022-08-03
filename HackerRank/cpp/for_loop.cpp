#include <iostream>
#include <cstdio>
#include <map>

int main() {
    int start, end;
    std::cin >> start >> end;

    std::map<int, std::string> dict = {
        {1, "one"},
        {2, "two"},
        {3, "three"},
        {4, "four"},
        {5, "five"},
        {6, "six"},
        {7, "seven"},
        {8, "eight"},
        {9, "nine"},
    };

    for (int i = start; i <= end; i++) {
        if (i > 9) {
            std::cout << (i % 2 == 0
                ? "even"
                : "odd") << std::endl;
        } else std::cout << dict[i] << std::endl;
    }

    return 0;
}
