//the sumOfAngles method takes in a shape with n number of angles 
//and returns the sum of all angles of the shape
class Main {
  public static void main(String[] s) {
    //will return 360
    int sumOfAngles = new AngleSum().sumOfAngles(4);
    System.out.println(sumOfAngles);
  }
}

class AngleSum {
  public int sumOfAngles(int n) {
    return 180 * (n - 2);
  }
}