import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class DigitalRoot {

  public static void main(String[] args) {
    System.out.println(new DigitalRoot().digital_root(234567));
  }

  public static int digital_root(int n) {
    List<String> digits = Arrays.asList(String.valueOf(n).split(""));
    List<Integer> digs = digits.stream().map(s -> Integer.parseInt(s)).collect(Collectors.toList());
    if(digs.size() == 1) {
      return n;
    }

    int sum = 0;

    for(int i = 0; i < digs.size(); i++) {
      sum += digs.get(i);
    }

    if(String.valueOf(sum).length() != 1) {
      return digital_root(sum);
    }
    return sum;
  }
}
