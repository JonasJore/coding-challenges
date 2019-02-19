/**
* This solution is horribly overengineered 0.0
* @Author: jonas jore
*/
public class NoZerosForHeroes {

  public static void main(String[] args) {
    System.out.println(noBoringZeros(960000));
    System.out.println(noBoringZerosRewrite(960000) + "\t <- rewrite here");
  }

  public static int noBoringZeros(int n) {
    if(n == 0) {
      return 0;
    }
    
    int trailingZeroes = 0;
    int nCopy = n;
    while(nCopy % 10 == 0) {
      ++trailingZeroes;
      nCopy /= 10;
    }
    
    String stringNum = "1";
    for(int i = 0; i < trailingZeroes; i++) {
      stringNum += "0";
    }
    
    int divisor = Integer.parseInt(stringNum);
    return n / divisor;
  }

  public static int noBoringZerosRewrite(int n) {
    if(n == 0) {
      return n;
    }
    while(n % 10 == 0) {
      n /= 10;
    }
    return n;
  }
}
