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

// Underscore in Numeric Literals
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
fn main() {
    let arr = [11, 22, 33];
    let i: usize = 2;
    print!("{}", arr[i]);
}