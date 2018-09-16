// TODO: logic is far from flawless! fix this later...

import java.lang.StringBuilder;

public class ReverseStringKeepSpacing {
  public static String reverseWords(final String original) {
    String[] words = original.split(" ");
    StringBuilder builder = new StringBuilder();
    for(int i = 0; i < words.length; i++) {
      StringBuilder sb = new StringBuilder(words[i]);
      words[i] = sb.reverse().toString();
      if(i == words.length-1) {
        builder.append(words[i]);
      } else {
        builder.append(words[i] + " ");
      }
    }

    return builder.toString();
  }
}
