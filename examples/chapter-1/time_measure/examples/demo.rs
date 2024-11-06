use time_measure::{start_timer, end_timer};

fn main() {
    let start = start_timer();

    // Example: Code block to measure
    for i in 0..1000000 {
        let _res: f64 = i as f64 * i as f64;
    }

    end_timer(start);
}