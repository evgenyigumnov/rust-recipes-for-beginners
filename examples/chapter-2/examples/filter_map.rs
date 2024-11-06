fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    
    let squares_of_even_numbers: Vec<i32> = numbers
        .into_iter()
        .filter(|&x| x % 2 == 0)  // Filter even numbers
        .map(|x| x * x)           // Square each filtered number
        .collect();
    
    println!("{:?}", squares_of_even_numbers);  // Output: [4, 16, 36]
}