fn main() {
    let large_range = 1..;
    let first_five_squares: Vec<i32> = large_range
        .map(|x| x * x) // Lazily creates an iterator of squares
        .take(5)        // Only takes the first 5 elements
        .collect();     // Now the computation is triggered

    println!("{:?}", first_five_squares);
}
