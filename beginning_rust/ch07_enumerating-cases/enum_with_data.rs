// Rust enumerations can work with data

/* fn main() {
    enum Result {
        Success(f64),
        Failure(u16, char),
        Uncertainty,
    }
    // let outcome = Result::Success(23.67);
    let outcome = Result::Failure(1200, 'X');
    match outcome {
        Result::Success(value) => print!("Result: {}", value),
        Result::Failure(error_code, module) => print!("Error n. {} in module {}", error_code, module),
        Result::Uncertainty => {},
    }
} */

// "match" expressions

/* fn main() {
    enum CardinalPoint { North, South, West, East };
    let direction = CardinalPoint::East;
    print!("{}", match direction {
        CardinalPoint::North => 'N',
        CardinalPoint::South => 'S',
        _ => '*',
    });
} */

// Use of Guards in match Constructs

fn main() {
    for n in -5..7 {
        println!("{} is {}.", n, match n {
            0 => "zero",
            1 => "one",
            _ if n < 0 => "negative",
            _ => "plural",
        })
    }
}