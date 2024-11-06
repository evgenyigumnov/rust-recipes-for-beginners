fn get_user_age(user_id: u32) -> Option<u32> {
    // Let's pretend we fetch the user, but user with id 1 doesn't exist.
    if user_id == 1 {
        None
    } else {
        Some(30) // Assume we fetched age 30 for other users.
    }
}
fn get_default_age() -> Option<u32> {
    Some(25)
}

fn main() {
    let age = get_user_age(1).or_else(get_default_age); // Some(25) because user 1 doesn't exist
    let valid_age = get_user_age(2).or_else(get_default_age); // Some(30)

    println!("{:?}, {:?}", age, valid_age); // Outputs: Some(25), Some(30)
}