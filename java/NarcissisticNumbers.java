class Main {
  public static void main(String[] args) {
    new Main().testisNarcissisticWillReturnTrue();
    new Main().testisNarcissisticWillReturnFalse();
  }

  void testisNarcissisticWillReturnTrue() {
    boolean test1 = new Kata().isNarcissistic(153);
    if(test1) {
      System.out.println("test passed");
    } else {
      System.out.println("test failed");
    }
  }

  void testisNarcissisticWillReturnFalse() {
    boolean test2 = new Kata().isNarcissistic(435);
    if(test2) {
      System.out.println("test passed");
    } else {
      System.out.println("test failed");
    }
  }
}

class Kata {
  public boolean isNarcissistic(int n) {
    String stringedNum = String.valueOf(n);
    int i = 0;
    int numDigs = String.valueOf(n).length();
    String[] arr = stringedNum.split("");
    Integer[] intArr = new Integer[arr.length];
    int sum = 0;
    for(String s : arr) {
      intArr[i] = Integer.parseInt(s);
      sum += Math.pow(intArr[i], numDigs);
      ++i;
    }
    return sum == n ? true : false;
  }
}
