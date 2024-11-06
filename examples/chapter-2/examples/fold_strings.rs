fn main() {
    let words = vec!["Rust", "is", "fun"];
    let sentence = words.iter().fold(String::new(), |mut acc, &word| {
        if !acc.is_empty() {
            acc.push(' ');
        }
        acc.push_str(word);
        acc
    });
    println!("{}", sentence);
}