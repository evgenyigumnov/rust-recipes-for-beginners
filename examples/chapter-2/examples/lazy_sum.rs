fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let total: i32 = numbers
        .iter()
        .map(|x| x * 2) // Lazily doubles each element
        .sum(); // Triggers the computation and sums the doubled values

    println!("Sum: {}", total);
}
