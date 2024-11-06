fn main() {
    let names = vec!["Alice", "Bob", "Charlie"];
    names
        .iter()
        .for_each(|name| println!("Hello, {}!", name)); // Lazily iterates and greets each name
}