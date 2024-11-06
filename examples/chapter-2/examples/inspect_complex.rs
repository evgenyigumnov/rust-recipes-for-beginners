fn main() {
    let data = vec![Some(1), None, Some(2), Some(3)];

// Using `inspect` to observe filtering and mapping
    let results: Vec<_> = data.into_iter()
        .inspect(|x| println!("Original: {:?}", x)) // Observe the raw data
        .filter(Option::is_some)
        .inspect(|x| println!("After filtering: {:?}", x)) // Observe filtered data
        .map(Option::unwrap)
        .inspect(|x| println!("After unwrapping: {}", x)) // Observe unwrapped values
        .collect();
    println!("{:?}", results);
}
