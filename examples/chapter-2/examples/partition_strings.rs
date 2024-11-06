fn partition_by_length(strings: Vec<String>) -> (Vec<String>, Vec<String>) {
    strings.into_iter().partition(|s| s.len() < 5)
}

fn main() {
    let words = vec!["apple".to_string(), "cat".to_string(), "banana".to_string(), "dog".to_string()];
    let (short_words, long_words) = partition_by_length(words);
    println!("Short words: {:?}", short_words);
    println!("Long words: {:?}", long_words);
}