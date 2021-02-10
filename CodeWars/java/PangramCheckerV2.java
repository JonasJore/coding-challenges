package no.jonjon;

import java.util.HashMap;
import java.util.Map;

public class Paners {

    private HashMap<Character, Integer> makeDictionary(String s) {
        HashMap<Character, Integer> dict = new HashMap<>();
        for (int i = 0; i < s.length(); i++) {
            dict.put(s.charAt(i), 0);
        }

        return dict;
    }

    /**
      This implementation will not check perfect pangrams
      only sentences which qualify for pangram where all letters
      is present atleast once
     */
    public boolean panerChecker(String s) {
        // s = thequickbrownfoxjumpsoverthelazydog
        final String NORWEGIAN_ALPHABET = "abcdefghijklmnopqrstuvwxyz";
        Map<Character, Integer> dictionary = makeDictionary(NORWEGIAN_ALPHABET);

        for (int i = 0; i < s.length(); i++) {
            dictionary.computeIfPresent(s.charAt(i), (k, v) -> v + 1);
        }

        return !dictionary.containsValue(0);
    }
}