fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    numbers.iter_mut().for_each(|x| *x *= 2);
    
    println!("{:?}", numbers);  // Output: [2, 4, 6, 8, 10]
}
