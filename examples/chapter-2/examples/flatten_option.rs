fn main() {
    let nested = Some(Some(42));

    let flattened = nested.flatten();

    println!("{:?}", flattened);
}