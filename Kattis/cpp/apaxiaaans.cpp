#include <iostream>
using namespace std;

int main(){
    string name;
    cin >> name;
    int last;
    for(auto c : name){
        if(c != last){
            cout << c;
            last = c;
        }
    }
    cout << endl;
    return 0;
}
