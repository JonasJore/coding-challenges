#include <iostream>
using namespace std;

int days_below_zero(int n){
	int days_below_zero_counter = 0;
	int temp;
	for(int i = 0; i < n; i++){
		cin>>temp;
		if(temp < 0){
			++days_below_zero_counter;
		}
	}
	return days_below_zero_counter;
}

int main(){
	int n;
	cin >> n;
	cout << days_below_zero(n);
	return 0;
}