fn main() {
    struct Person {
        name: String,
        age: u8,
    }

    let mut people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];

    // Sorting by age in ascending order
    people.sort_by(|a, b| a.age.cmp(&b.age));

    for person in people {
        println!("{} is {} years old", person.name, person.age);
    }
}
