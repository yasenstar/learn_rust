/* "tuples" vs. "structs":
- Tuples, whose types have no name and are not to be previously declared, and whose fields have no name;
- Structs, whose types have a name, and must be previously declared, and whose fields have a name
*/

// hybrid: "tuple-structs"
/* fn main() {
    struct SomeData (
        i32,
        f32,
        char,
        [u8; 5],
    );
    let data = SomeData (
        10_000_000,
        183.19,
        'Q',
        [9, 0, 250, 60, 200],
    );
    println!("{} {} {} {}", data.2, data.0, data.1, data.3[2]);
} */

// The "tuple-struct" is defined before instantiating it,
// by using the keyword "struct" like a struct,
// but enclosing its fields in parentheses,
// and without specifying the names of the fields, like a tuple.
// The initialization starts with the name of the type,
// like a struct, but goes on like a tuple.
// Its fields are accessed, by necessity, like a tuple, as they have no name.
// Differing from both tuples and structs, empty tuple-structs are not allowed.
// Tuple-structs are not actually used often.
