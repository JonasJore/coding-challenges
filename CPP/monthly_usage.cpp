#include <iostream>
using namespace std;
int main(){
	int monthly_Usage, months, total = 0;
	cin >> monthly_Usage >> months;
	int total_Usage = 0;
	for(int i = 0; i < months; i++){
		int next;
		cin >> next;
		total_Usage += next;
	}
	
	cout << (months + 1) * monthly_Usage - total_Usage << endl;
	
	return 0;
}