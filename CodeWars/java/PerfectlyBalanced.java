import java.util.List;
import java.util.stream.Collectors;

public class PerfectlyBalanced{

  public static void main(String[] args){
    boolean xxxyyy = new PerfectlyBalanced().PerfectlyBalanced("yxyxyxyxyxyxyxyxyxyxyxyxyxyx");
    System.out.println("Balanced:\t" + xxxyyy);
  }

  public boolean PerfectlyBalanced(String inputString){
    List<Character> characterList = inputString.chars().mapToObj(c -> (char) c).collect(Collectors.toList());
    int x = 0;
    int y = 0;
    for(Character ch : characterList){
      if(ch.equals('x')){
        x += 1;
      } else if(ch.equals('y')){ // added due to more robust checking when others chars than x and y appears
        y += 1;
      }
    }

    return x == y;
  }

}
