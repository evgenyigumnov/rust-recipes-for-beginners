fn main() {
    let data = vec![Some(1), None, Some(3), Some(4)];

    let flattened: Vec<i32> = data.into_iter().flat_map(|x| x).collect();

    println!("{:?}", flattened);
}