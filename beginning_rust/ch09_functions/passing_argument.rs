// function print the sum of any two numeric values

/* fn main() {
    fn print_sum(addend1: f64, addend2: f64) {
        println!("{} + {} = {}", addend1, addend2, addend1 + addend2);
    }

    print_sum(3., 5.);
    print_sum(3.2, 5.1);
} */

/* result:
3 + 5 = 8
3.2 + 5.1 = 8.3
*/

// passing arguments by value

/* fn main() {
    fn print_double(mut x: f64) {
        x *= 2.;
        print!("{}", x);
    }
    let x = 4.;
    print_double(x);
    print!(" {}", x);
} */

// returning a value from function

/* fn main() {
    fn double(x: f64) -> f64 { x * 2. }
    print!("{}", double(17.3));
} */

// note: the value of any block is the value of its last expression, if there's a last expression, or otherwise an empty tuple.

// Early Exit from the function

/* fn main() {
    fn f(x: f64) -> f64 {
        if x <= 0. { return 0.; }
        x + 3.
    }
    print!("{} {} {}", f(1.), f(-5.), f(100.));
} */

// note: "return" statement evaluates the expression that follows it, and the resulting value is immediately returned to the caller.

// return several (more than 1) values
// can return a tuple, an enum, a struct, a tuple struct, an array, or a vector

/* fn main() {
    fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
        (dividend / divisor, dividend % divisor)
    }
    let x = 100;
    let y = 8;
    for i in 1..y {
        println!("{} / {} = {:?}", x, i, divide(x,i));
    }
} */

// Change several values in the array
/* fn main() {
    let mut arr = [ 5, -4, -9, 7, -1, 4, -8, -3, 2];
    println!("{:?}", arr);
    for i in 1..arr.len() {
        if arr[i] < 0 { arr[i] *= 2; }
    }
    println!("{:?}", arr);
} */
// using function:
fn main() {
    fn double_negatives(mut a: [i32; 9]) -> [i32; 9] {
        for i in 0..9 {
            if a[i] < 0 { a[i] *= 2; }
        }
        a
    }
    let mut arr = [ 5, -4, -9, 7, -1, 4, -8, -3, 2];
    arr = double_negatives(arr);
    println!("{:?}", arr);
}