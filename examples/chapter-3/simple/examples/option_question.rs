fn first_char_of_line(text: &str) -> Option<char> {
    let line = text.lines().next()?;       // Returns None if there are no lines
    let ch = line.chars().next()?;         // Returns None if the line is empty
    Some(ch)
}

fn main() {
    let text = "Hello\nWorld";
    if let Some(ch) = first_char_of_line(text) {
        println!("First character: {}", ch);
    } else {
        println!("No characters found");
    }
}