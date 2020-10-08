/* fn main() {
    struct SomeData {
        integer: i32,
        fractional: f32,
        character: char,
        five_bytes: [u8; 5],
    }
    let data = SomeData {
        integer: 10_000_000,
        fractional: 183.19,
        character: 'Q',
        five_bytes: [9, 0, 250, 60, 200],
    };
    println!("{} {} {} {}", data.five_bytes[3], data.integer, data.fractional, data.character);
    // println!("{:?}", data);
} */

/* formatting error when printing with :?
println!("{:?}", data);
   |                      ^^^^ `main::SomeData` cannot be formatted using `{:?}`
*/

// If you declare a variable as mutalbe,
// you can change the values of its fields, using dot-notation

/* fn main() {
    struct SomeData {
        integer: i32,
        fractional: f32,
    }
    let mut data = SomeData {
        integer: 10,
        fractional: 183.19,
    };
    println!("{} {}", data.fractional, data.integer);
    data.fractional = 8.2;
    println!("{} {}", data.fractional, data.integer);
} */
