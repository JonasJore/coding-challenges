// https://www.codewars.com/kata/5264d2b162488dc400000001/train/rust

fn spin_words(words: &str) -> String {
    let mut result = "".to_string();
    for word in words.split(' ') {
        if word.len() >= 5 {
            let reversed_word = &word.chars().rev().collect::<String>();
            result.push_str(&(reversed_word.to_owned() + " "));
        } else {
            result.push_str(&(word.to_owned() + " "))
        }
    }

    result.trim_right().to_string()
}

// these match expressions are so clean!
fn spin_words_match(words: &str) -> String {
    return words
        .split_ascii_whitespace()
        .map(|word| match word.len() >= 5 {
            true => word.chars().rev().collect(),
            false => word.to_string(),
        })
        .collect::<Vec<String>>()
        .join(" ");
}

fn main() {
    let test = "This string has some words reversed!";
    let res = spin_words_match(test);
    println!("{}", res);
}

#[test]
fn examples() {
    assert_eq!(spin_words("Welcome"), "emocleW");
    assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
    assert_eq!(spin_words("This is a test"), "This is a test");
    assert_eq!(spin_words("This is another test"), "This is rehtona test");
    assert_eq!(
        spin_words("You are almost to the last test"),
        "You are tsomla to the last test"
    );
    assert_eq!(
        spin_words("Just kidding there is still one more"),
        "Just gniddik ereht is llits one more"
    );
    assert_eq!(
        spin_words("Seriously this is the last one"),
        "ylsuoireS this is the last one"
    );
}
