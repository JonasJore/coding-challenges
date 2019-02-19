/**
* This solution is horribly overengineered 0.0
* @Author: jonas jore
*/
public class NoZerosForHeroes {

  public static void main(String[] args) {
    System.out.println(noBoringZeros(-1050));
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
    // return nCopy;
  }
}
