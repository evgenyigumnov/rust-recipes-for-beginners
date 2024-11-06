fn main() {
    let mut items = vec![
        ("apple", 2),
        ("banana", 1),
        ("orange", 3)
    ];

// Sorting by the second value (quantity)
    items.sort_by(|a, b| a.1.cmp(&b.1));
    println!("{:?}", items); // Output: [("banana", 1), ("apple", 2), ("orange", 3)]
}
