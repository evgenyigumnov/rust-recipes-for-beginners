fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let even_numbers: Vec<&i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .collect();

    println!("{:?}", even_numbers);  // Output: [&2, &4]

    // The `numbers` vector can still be used here
    println!("{:?}", numbers);  // Output: [1, 2, 3, 4, 5]
}