fn partition_even_odd(numbers: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    numbers.into_iter().partition(|&n| n % 2 == 0)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let (evens, odds) = partition_even_odd(numbers);
    println!("Even numbers: {:?}", evens);
    println!("Odd numbers: {:?}", odds);
}