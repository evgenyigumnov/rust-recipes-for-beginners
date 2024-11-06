fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squared_numbers: Vec<i32> = numbers
        .iter()
        .map(|x| x * x) // Lazily creates an iterator that squares each element
        .collect(); // Now the map operation is evaluated and collected into a Vec

    println!("{:?}", squared_numbers);
}