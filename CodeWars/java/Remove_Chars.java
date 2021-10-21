//https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0

public class RemoveChars {

  public static void main(String[] args) {
		new RemoveChars().tester();
	}

  private void tester() {
		Scanner sc = new Scanner(System.in);
		int tests = sc.nextInt();
		for(int i = 0; i < tests; i++) {
			String inputTests = sc.nextLine();
			String removedChars = remove(inputTests);
			System.out.println(checker(inputTests, removedChars));
		}
	}

  private String checker(String inputWord, String removedWord) {
		if((inputWord == "eloquent" && removedWord == "loquen") || 
       (inputWord == "country" && removedWord == "ountr") ||
       (inputWord == "person" && removedWord == "erso") ||
       (inputWord == "place" && removedWord == "lac") {
			return "Yes";
		} 
		return "No";
	}
    
  public static String remove(String str) {

    String newStr = "";
    for(int i = 1; i < str.length()-1; i++){
      newStr += str.charAt(i); 
    }
    
    return (newStr);
  }
}