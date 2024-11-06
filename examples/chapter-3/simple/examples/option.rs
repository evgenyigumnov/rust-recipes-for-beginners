fn find_number(numbers: &[i32], target: i32) -> Option<usize> {
    for (index, &number) in numbers.iter().enumerate() {
        if number == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let numbers = vec![10, 20, 30, 40];
    match find_number(&numbers, 30) {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }
}