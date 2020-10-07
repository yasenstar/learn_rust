// Enumerations

// Without using enumerations
/* fn main() {
    const EUROPE: u8 = 0;
    const ASIA: u8 = 1;
    const AFRICA: u8 = 2;
    const AMERICA: u8 = 3;
    const OCEANIA: u8 = 4;

    let continent = OCEANIA;

    if continent == EUROPE { print!("E"); }
    else if continent == ASIA { print!("As"); }
    else if continent == AFRICA { print!("Af"); }
    else if continent == AMERICA { print!("Am"); }
    else if continent == OCEANIA { print!("O"); }
} */

// With enumerations
// This type is called "enumerative", it's possible to create objects of such type with name "enumerations" or "enum".
/* fn main() {
    enum Continent {
        Europe,
        Asia,
        Africa,
        America,
        Oceania,
    }

    let contin = Continent::Europe;

    match contin {
        Continent::Europe => print!("E"),
        Continent::Asia => print!("As"),
        Continent::Africa => print!("Af"),
        Continent::America => print!("Am"),
        Continent::Oceania => print!("O"),
    }
} */

// The match Construct
// The "match" statement is the basic Rust tool to use enumerations, similarly to the "switch" statemewnt in the C language.

// NOTE: Emums are not comparalbe using the Relational Operators, e.g "==", "<", etc..

// Handle all the cases
/* fn main() {
    enum CardinalPoint { North, South, East, West };
    let direction = CardinalPoint::South;
    match direction {
        CardinalPoint::North => print!("NORTH"),
        CardinalPoint::East => {},
        CardinalPoint::West => {},
        CardinalPoint::South => print!("SOUTH"),
    }
} */

// To avoid having to list all the variants that do nothing, it is possible to use an underscore sign, in the following way:
/* fn main() {
    enum CardinalPoint { North, South, East, West };
    let direction = CardinalPoint::South;
    match direction {
        CardinalPoint::North => print!("NORTH"),
        CardinalPoint::South => print!("SOUTH"),
        _ => {},
    }
} */
// explaination: the underscore sign always matches with any value. Note: the "_" clause must be the last statement within Match

// Using match with Numbers
fn main() {
    match "value" {
        "val" => print!("value "),
        _ => print!("other "),
    }
    match 3 {
        5 => print!("five "),
        4 => print!("four "),
        3 => print!("three "),
        _ => print!("other "),
    }
    match '.' {
        ':' => print!("colon "),
        '.' => print!("point "),
        _ => print!("other "),
    }
}