struct Person {
    name: String,
    age: u32,
}

fn main() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 40 },
        Person { name: "Charlie".to_string(), age: 25 },
    ];
    
    let adults: Vec<&Person> = people
        .iter()
        .filter(|&person| person.age >= 30)  // Keep only people aged 30 or above
        .collect();
    
    for person in adults {
        println!("{} is an adult", person.name);
    }
}