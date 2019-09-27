import java.util.Set;
import java.util.HashSet;

class Main {
  public static void main(String[] args) {
    System.out.println(new Main().check("The quick brown fox jumps over the lazy dog."));
  }

  public boolean check(String sentence){
    if(sentence.length() < 26) {
      return false;
    }
    String theSentence = sentence.toLowerCase().replaceAll("[^a-zA-Z]", "");
    char[] chars = theSentence.toCharArray();
    final Set<Character> set = new HashSet<Character>();
    for(char c:chars){
      set.add(c);
    }

    return (set.size() == 26);
  }

}
