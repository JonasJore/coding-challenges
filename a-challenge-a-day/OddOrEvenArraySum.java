import java.util.stream.IntStream;

class Main {
  public static void main(String[] args) {
    System.out.println(oddOrEven(new int[] {2,2}));
  }

  public static String oddOrEven (int[] array) {
    return (IntStram.of(array).sum() % 2 == 0) ? "even" : "odd";
  }
}
