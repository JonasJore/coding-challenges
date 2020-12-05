fn is_vowel(s: &char) -> bool {
 let new_char = s.to_lowercase().collect::<Vec<_>>();
 !(new_char[0] == 'a' || new_char[0] == 'e' || new_char[0] == 'i' || new_char[0] == 'o' || new_char[0] == 'u')
}

fn dissemvoweled(s: &str) -> String {
    s.chars().into_iter()
        .filter(|char| is_vowel(&char))
        .collect::<Vec<char>>().iter().cloned()
        .collect::<String>()
}


fn main() {
    let s = "This website is for losers LOL!";

    let dissemvoweled: String = dissemvoweled(s);

    println!("{}", dissemvoweled);
}
