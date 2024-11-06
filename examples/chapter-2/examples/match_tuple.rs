fn process_tuple(pair: (i32, i32)) -> i32 {
    match pair {
        (x, y) if x == y => x * 2,
        (x, y) => x + y,
    }
}
fn main() {
    let result1 = process_tuple((3, 3)); // 6
    let result2 = process_tuple((2, 3)); // 5
    println!("{}, {}", result1, result2);
}