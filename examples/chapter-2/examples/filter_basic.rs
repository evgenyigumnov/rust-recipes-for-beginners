fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let even_numbers: Vec<i32> = numbers
        .into_iter()
        .filter(|&x| x % 2 == 0)  // Keep only even numbers
        .collect();

    println!("{:?}", even_numbers);  // Output: [2, 4, 6]
}