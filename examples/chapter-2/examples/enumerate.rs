fn main() {
    let words = vec!["apple", "banana", "cherry", "date"];

    // Use `enumerate` to pair each item with its index
    for (index, word) in words.iter().enumerate() {
        println!("Item {} is {}", index, word);
    }
}
