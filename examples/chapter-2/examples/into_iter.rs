fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    let doubled_numbers: Vec<i32> = numbers
        .into_iter()
        .map(|x| x * 2)
        .collect();
    
    // Cannot use `numbers` here anymore, because it has been consumed by `into_iter()`
    println!("{:?}", doubled_numbers);  // Output: [2, 4, 6, 8, 10]
}