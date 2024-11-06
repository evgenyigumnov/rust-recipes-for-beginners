fn main() {
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![85, 92, 78];

    let paired_data = names.iter().zip(scores.iter());

    for (name, score) in paired_data {
        println!("{} scored {}", name, score);
    }
}