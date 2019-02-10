public class Line {

  // Syntactic sugar
  private static final String YES = "YES";
  private static final String NO = "NO";

  public static void main(String[] args) {
    String tickets = new Line().Tickets(new int[] {25, 25, 25, 25, 50, 100, 50});
    System.out.println(tickets);
  }

  public static String Tickets(int[] peopleInLine) {
    int numBills25 = 0;
    int numBills50 = 0;
    int numBills100 = 0;

    for(int i = 0; i < peopleInLine.length; i++) {
      if(peopleInLine[i] == 100) {
        if(numBills25 >= 3) {
          numBills25 -= 3;
          ++numBills100;
        } else if(numBills25 >= 1 && numBills50 >= 1) {
          --numBills25;
          --numBills50;
          ++numBills100;
        } else {
          return NO;
        }
      }

      if(peopleInLine[i] == 50) {
        if(numBills25 >= 1) {
          --numBills25;
          ++numBills50;
        } else if(numBills50 >= 1) {
          --numBills50;
          ++numBills50;
        } else {
          return NO;
        }
      }

      if(peopleInLine[i] == 25) {
        ++numBills25;
      }
    }

    return YES;
  }
}
