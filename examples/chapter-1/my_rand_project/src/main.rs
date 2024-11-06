// Importing necessary traits and structs from the `rand` crate.
use rand::Rng; // Trait for random number generation methods.
use rand::rngs::SmallRng; // A small, fast pseudo-random number generator.
use rand::SeedableRng; // Trait that allows creating a random number generator from a seed.

fn main() {
    // Create an instance of `SmallRng`, seeded from the system's entropy source.
    // `from_entropy()` provides a convenient way to initialize the RNG with randomness from the system.
    let mut rng = SmallRng::from_entropy();

    // Generate a random number of type `u8` in the range 1 to 100 (inclusive of 1 and exclusive of 101).
    let random_number: u8 = rng.gen_range(1..101);

    // Print the random number to the console.
    println!("Random number with SmallRng: {}", random_number);
}

#[cfg(test)] // This marks the following module as a test module, which will only be compiled in test mode.
mod tests {
    use super::*; // Import all items from the parent scope for testing.
    use rand::rngs::SmallRng; // Importing `SmallRng` for use in the test.
    use rand::SeedableRng; // Importing `SeedableRng` to seed the RNG.

    #[test] // This attribute marks the following function as a test.
    fn test_random_number_with_small_rng() {
        // Create an instance of `SmallRng` seeded from system entropy, just like in the main function.
        let mut rng = SmallRng::from_entropy();

        // Generate a random number of type `u8` in the range 1 to 100.
        let random_number: u8 = rng.gen_range(1..101);

        // Assert that the generated number falls within the expected range (inclusive 1, exclusive 101).
        assert!(random_number >= 1 && random_number <= 100);
    }
}
