//https://www.codewars.com/kata/525f50e3b73515a6db000b83/train/java
import org.junit.Test;
import static org.junit.Assert.assertEquals;

public class Kata {
  public static String createPhoneNumber(int[] numbers) {
    return("(" + numbers[0] + numbers[1] + numbers[2] + ") " + numbers[3] + numbers[4] + numbers[5] + "-" + numbers[6] + numbers[7] + numbers[8] + numbers[9]);
  }
}

public class PhoneExampleTests {
  @Test
  public void tests() {
  assertEquals("(123) 456-7890", Kata.createPhoneNumber(new int[] {1, 2, 3, 4, 5, 6, 7, 8, 9, 0}));
  assertEquals("(987) 654-3210", Kata.createPhoneNumber(new int[] {9, 8, 7, 6, 5, 4, 3, 2, 1, 0}));
  }
}