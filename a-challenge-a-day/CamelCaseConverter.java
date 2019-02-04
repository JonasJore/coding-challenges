import java.util.Arrays;
import java.util.stream.Collectors;

public class CamelCaseConverter {

  public static void main(String[] args) {
    String test = new CamelCaseConverter().camelCase("camel case word");
    System.out.println(test);
    String test2 = new CamelCaseConverter().camelCase(" hello world ");
    System.out.println(test2);
    String test3 = new CamelCaseConverter().camelCase("Camelize this");
    System.out.println(test3);
  }

  public static String camelCase(String str) {
    return String.join("", Arrays.stream(str.split(" "))
        .filter(item -> !item.isEmpty())
        .map(item -> item.substring(0, 1).toUpperCase() + item.substring(1))
        .collect(Collectors.toList()));
  }
}
