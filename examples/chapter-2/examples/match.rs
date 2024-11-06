enum Direction {
    North,
    South,
    East,
    West,
}

fn describe_direction(dir: Direction) {
    match dir {
        Direction::North => println!("You are heading North."),
        Direction::South => println!("You are heading South."),
        Direction::East => println!("You are heading East."),
        Direction::West => println!("You are heading West."),
    }
}

fn main() {
    let direction = Direction::North;
    describe_direction(direction); // Outputs: You are heading North.
}