//takes in an int from scanner object
//and returns given number translated to its binary int value

import java.util.Scanner;

class Main {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    int n = sc.nextInt();
    int bin = new Main().toBinary(n);
    System.out.println(bin);
  }
  
  public static int toBinary(int n) {
    String binaryNumberString = "";
    do {
      if(n % 2 == 0) {
        binaryNumberString = '0' + binaryNumberString;
      } else {
        binaryNumberString = '1' + binaryNumberString;
      }
      n /= 2;
    } while(n != 0);
    return Integer.parseInt(binaryNumberString);
  }
  
}
