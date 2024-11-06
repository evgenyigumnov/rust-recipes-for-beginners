fn get_user_age(user_id: u32) -> Option<u32> {
    // Let's pretend we fetch the user, but user with id 1 doesn't exist.
    if user_id == 1 {
        None
    } else {
        Some(30) // Assume we fetched age 30 for other users.
    }
}

fn double_age(user_id: u32) -> Option<u32> {
    get_user_age(user_id).and_then(|age| Some(age * 2))
}

fn main() {
    let age = double_age(2); // Some(60)
    let missing_age = double_age(1); // None

    println!("{:?}, {:?}", age, missing_age); // Outputs: Some(60), None
}