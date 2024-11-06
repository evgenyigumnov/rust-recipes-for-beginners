fn main() {
    let nested = vec![
        vec![1, 2, 3],
        vec![4, 5],
        vec![6, 7, 8, 9],
    ];

    let flattened: Vec<i32> = nested.into_iter().flatten().collect();

    println!("{:?}", flattened);
}