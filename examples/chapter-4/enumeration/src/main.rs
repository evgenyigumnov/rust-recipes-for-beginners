use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Cli {
    /// Choose a language
    #[arg(value_enum)]
    language: Language,
}

#[derive(ValueEnum, Clone)]
enum Language {
    English,
    Spanish,
    French,
}

fn main() {
    let args = Cli::parse();

    let greeting = match args.language {
        Language::English => "Hello!",
        Language::Spanish => "¡Hola!",
        Language::French => "Bonjour!",
    };

    println!("{}", greeting);
}