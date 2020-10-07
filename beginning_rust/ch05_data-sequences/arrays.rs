// Arrays (by default, the Array is not mutable)
/* fn main() {
    let x = ["English", "This", "sentence", "a", "in", "is"];
    println!("{} {} {} {} {} {}", x[1], x[5], x[5], x[2], x[4], x[0]);
} */

// determine how many elements are in an Array
/* fn main() {
    let a = [true, false];
    let b = [1, 2, 3, 4, 5, 6, 7];
    println!("{} {}", a.len(), b.len());
} */
// Note: every array contains element of the SAME type!

// Mutable Array: the modification of the items of an arry is possible only on mutable arrays
/* fn main() {
    let mut x = ["This", "is", "a", "sentence"];
    println!("{} {} {} {}", x[0], x[1], x[2], x[3]);
    x[2] = "a nice";
    println!("{} {} {} {}", x[0], x[1], x[2], x[3]);
} */

// Handle Array with many items
/* fn main() {
    let mut x = [4.; 5000];
    x[2000] = 3.1415926;
    print!("{} {}", x[4500], x[2000]);
} */

// Scan the items of an array using "for" statemewnts: Fibonacci Sequence
/* fn main() {
    let mut fib = [1.; 50];
    for i in 2..fib.len() {
        fib[i] = fib[i-2] + fib[i-1];
    }
    for i in 0..fib.len() {
        print!("{}, ", fib[i]);
    }
} */

// Multi-dimensional Arrays
/* fn main() {
    let mut x = [[[[23;4]; 6]; 8]; 15];
    x[14][7][5][3] = 56;
    println!("{} {}", x[0][0][0][0], x[14][7][5][3]);
    print!("{} {} {} {}", x.len(), x[0].len(), x[0][0].len(), x[0][0][0].len());
} */
// Note: the big limitation of arrays is the fact that their size must be defined at compilation time.
/* see below sample:
   let length = 6;
   let arr = [0; length];
The compilation of this code generates the error "attempt to use a non-constant value in a constant".
Actually the expression "length" is a variable, and therefor conceptually it is not a compile-time constant,
even if it is immutable, and even if it has just been initialized by a constant.
The size of an arry cannot be an expression containing variables.
 */