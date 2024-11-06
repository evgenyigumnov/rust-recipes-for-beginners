fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = numbers.iter().fold(0, |acc, &num| acc + num);
    println!("The sum is: {}", sum);
}