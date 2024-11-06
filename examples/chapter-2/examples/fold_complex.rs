fn main() {
    let numbers = vec![2, 0, 3, 4];
    let product = numbers.iter().fold(1, |acc, &num| {
        if num == 0 {
            acc // Skip zeros by not changing the accumulator
        } else {
            acc * num
        }
    });
    println!("The product is: {}", product);
}