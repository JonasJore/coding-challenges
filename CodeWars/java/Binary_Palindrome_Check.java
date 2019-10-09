import java.util.Scanner;
public class Binary_Palindrome_Check {
	
	public static void main(String[] args) {
		new Binary_Palindrome_Check().tester();
	}

	private void tester() {
		Scanner sc = new Scanner(System.in);
		int tests = sc.nextInt();
		for(int i = 0; i < tests; i++) {
			int inputTests = sc.nextInt();
			String binString = to_bin(inputTests);
			System.out.println(palChecker(binString, reverseString(binString)));
		}
	}

	private String palChecker(String word, String reversedWord) {
		if(word.equals(reversedWord)) {
			return "Yes";
		} 
		return "No";
	}

	private String reverseString(String theString) {
		char[] chars = theString.toCharArray();
		int begin = 0;
		int end = chars.length-1;
		char tmp;
		while(end > begin) {
			tmp = chars[begin];
			chars[begin] = chars[end];
			chars[end] = tmp;
			end--;
			begin++;
		}
		return new String(chars);
	}
	
	private String to_bin(int n) {
		String bin = "";
		do {
			if(n % 2 == 0) {
				bin = '0' + bin;
			} else {
				bin = '1' + bin;
			}
			n /= 2;
		} while(n != 0);
		return bin;
	}
}
