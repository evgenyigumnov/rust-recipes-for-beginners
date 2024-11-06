use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("my-cli-uppercase")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Greets a user")
        .arg(
            Arg::new("name")
                .help("Name of the user")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("uppercase")
                .help("Display the greeting in uppercase")
                .short('u')
                .long("uppercase")
                .global(true)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let name = matches.get_one::<String>("name").unwrap();
    let greeting = format!("Hello, {}!", name);

    let uppercase = matches.get_one::<bool>("uppercase").unwrap();
    if *uppercase {
        println!("{}", greeting.to_uppercase());
    } else {
        println!("{}", greeting);
    }
}