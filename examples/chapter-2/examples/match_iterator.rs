fn main() {
    let values = vec![Some(1), None, Some(3), None, Some(5)];

    let sum: i32 = values
        .into_iter()
        .filter_map(|x| match x {
            Some(v) => Some(v),
            None => None,
        })
        .sum();

    println!("The sum is: {}", sum); // Outputs: The sum is: 9
}