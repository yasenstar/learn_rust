// The Empty Tuple - "()"
// This type has only one value, which is written in the same way as its type, which is "()".
// This type somewhat corresponds to the "void" type of the C language, or to the "undefined" type of JavaScript.
// It is named "empty tuple".

/* all following cases except b represents Empty Tuple type:
    let a: () = ();
    let b = { 12; 87; 283 }  // b = 283 then.
    let c = { 12; 87, 283; }  // notice the end of the 283 has one ";"
    let d = {};
    let e = if false { };  // meant to be "let e = if false { } else { }"
    let f = while false { };
*/

// Array and Vector Types

// Explicit declare the type of Arrays or Vectors:
/* fn main() {
    let _array1: [char; 3] = ['x', 'y', 'z'];
    let _array2: [f32; 200] = [0f32; 200];
    let _vector1: Vec<char> = vec!['x', 'y', 'z'];
    let _vector2: Vec<i32> = vec![0; 5000];
} */

// Const: allow us to declare an identifier having a value defined at compile time,
// and of course no more changeable at runtime.
fn main() {
    const N: usize = 20;
    let _ = [0; N];
}