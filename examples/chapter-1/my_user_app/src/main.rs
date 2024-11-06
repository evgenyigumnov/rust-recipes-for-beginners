use my_user_library::User;

fn main() {
    let user = User{ name: "Alice".to_string(), age: 0 };
    let json = user.to_json();
    println!("Serialized to JSON: {}", json);
}