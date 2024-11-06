fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Chain of transformations with inspect to peek at the intermediate steps
    let squared_numbers: Vec<i32> = numbers.iter()
        .inspect(|&x| println!("Before squaring: {}", x)) // Inspect original values
        .map(|x| x * x)
        .inspect(|&x| println!("After squaring: {}", x))  // Inspect squared values
        .collect();
    println!("{:?}", squared_numbers);
}
