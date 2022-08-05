#include <iostream>
#include <cstdio>
using namespace std;

int max_of_four(int a, int b, int c, int d) {
    int arr[] = {a, b, c, d};
    int max = 0;
    for (int i = 0; i < 4; i++) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }

    return max;
}

int main() {
    int a, b, c, d;
    scanf("%d %d %d %d", &a, &b, &c, &d);
    int ans = max_of_four(a, b, c, d);
    printf("%d", ans);
    return 0;
}
