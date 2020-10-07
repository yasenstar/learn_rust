// Chapter 06

// Non-Decimal Numeric Bases
/* fn main() {
    let hexadecimal = 0x1000;
    let decimal = 1000;
    let octal = 0o1000;
    let binary = 0b1000;
    println!("{} {} {} {}", hexadecimal, decimal, octal, binary);
} */
/* Note:
The numbers of this sample are expressed in different notations - 0x, 0o, 0b,
but they are all of the same type: integer numbers. */

// Undeirscore in Numeric Literals
// The underscore characters ("_") can be inserted in any literal number, even floating point, and they are ignored by the compiler.
/* fn main() {
    let hexadecimal = 0x_00FF_F7A3;
    let decimal = 1_234_567_890;
    let octal = 0o_777_205_162;
    let binary = 0b_0110_1011_1111_0101_1010;
    println!("{} {} {} {}", hexadecimal, decimal, octal, binary);
} */
/* note for the range of integer:
   = note: the literal `0x_00FF_F7A3_EF9F` (decimal `1099371376543`) does not fit into the type `i32` and will become `-140251233i32`
   = help: consider using `i64` instead
*/

// Exponential Notation
// number before "e" is named "mantissa", while the number following "e" is named "exponent".
/* fn main() {
    let one_thousand = 1e3;
    let one_million = 1e6;
    let thirteen_billions_and_half = 13.5e9;
    let twelve_millionths = 12e-6;
    println!("{} {} {} {}", one_thousand, one_million, thirteen_billions_and_half, twelve_millionths);
} */

// Various Kinds of Signed Integer Numbers in Rust Language
// i8, i16, i32, i64: 8-bit / 16-bit / 32-bit / 64-bit signed integer number, "i" is shorthand for "integer"

// Unsigned Integer Number Types
// u8, u16, u32, u64: 8-bit / 16-bit / 32-bit / 64-bit unsigned integer number, "u" is shorthand for "unsigned"

// Target-Dependent Integer-Number Types
// The index of an array or vector should be unsigned, and it should have the same size of a memory address
// Now, Rust is not supported for 16-bit systems, but it's both for 32-bit and 64-bit systems.
// There is a need to specify an integer number type having a size dependent on the target, which is
// a 32-bit integer if the target is a 32-bit system, and a 64-bit integer if the target is a 64-bit system.
// To such purpose, Rust contains the "isize" type and the "usize" type:
//     let arr = [11, 22, 33];
//     let i: usize = 2;
//     print!("{}", arr[i]);
// }

// fn main() {
    //     let i = 8;
    //     let j = 8_000_000_000;
    //     print!("{} {}", i, j);
    // }
    
    // Explicit Conversion
    // fn main() {
        //     let a: i16 = 12;
        //     let b: u32 = 4;
        //     let c: f32 = 3.9;
        //     print!("{}", a as i8 + b as i8 + c as i8);
        // }
        
/*         fn main() {
            let a = 500 as i8;
            let b = 100_000 as u16;
            let c = 10_000_000_000 as u32;
            print!("{} {} {}", a, b, c);
        } */
        
        // fn main() {
// following are the error message:
// error: literal out of range for `i8`
//   --> primitive_types.rs:72:13
//    |
// 72 |     let a = 500 as i8;
//    |             ^^^
//    |
//    = note: the literal `500` does not fit into the type `i8` whose range is `-128..=127`

// error: literal out of range for `u16`
//   --> primitive_types.rs:73:13
//    |
//    = note: `#[deny(overflowing_literals)]` on by default
// 73 |     let b = 100_000 as u16;
//    |             ^^^^^^^
//    |
//    = note: the literal `100_000` does not fit into the type `u16` whose range is `0..=65535`

// error: literal out of range for `u32`
//   --> primitive_types.rs:74:13
//    |
// 74 |     let c = 10_000_000_000 as u32;
//    |             ^^^^^^^^^^^^^^
//    |
//    = note: the literal `10_000_000_000` does not fit into the type `u32` whose range is `0..=4294967295`

// error: aborting due to 3 previous errors

// all the Numeric Types
/*
let _: i8 = 127;
let _: i16 = 32_767;

*/