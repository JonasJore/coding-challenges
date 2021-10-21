//https://www.codewars.com/kata/514b92a657cdc65150000006/train/java

public class Solution {

  public static void main(String[] args) {
		new Solution().tester();
	}

	private void tester() {
		Scanner sc = new Scanner(System.in);
		int tests = sc.nextInt();
		for(int i = 0; i < tests; i++) {
			int inputTests = sc.nextInt();
			int sum = solution(inputTests);
			System.out.println(checker(inputTests, sum));
		}
	}

  private String checker(int inp, int sum) {
		if((inp == 10 && sum == 23) || (inp == 16 && sum == 60)) {
			return "Yes";
		} 
		return "No";
	}

  public int solution(int number) {
    int sum = 0;
    for(int i = 0; i < number; i++){
      if(i % 3 == 0 || i % 5 == 0){
        sum += i;
      }
    }
    return(sum);
  }
}