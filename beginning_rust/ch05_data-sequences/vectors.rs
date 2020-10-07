// Definition of Vector
// To create sequences of objects whose size is defined in runtime,
// the Rust standard library provides the "Vec" type, shorthand for "vector".
// Vector allows us to do everything that is allowed for Arrays,
// but they allow also us to change their size after having been initialized.

/* fn main() {
    let x = vec!["This", "is"];
    print!("{} {}. Length: {}.", x[0], x[1], x.len());
} */
// The clause "ve!" is an invocation of the "vec" macro of the standard library.

// change size after initialized
/* fn main() {
    let mut x = vec!["This", "is"];
    println!("{}", x.len());
    x.push("a");
    println!("{}", x.len());
    x.push("sentence");
    println!("{}", x.len());
    for i in 0..x.len() {
        print!("{} ", x[i]);
    }
    x[0] = "That";
    for i in 0..x.len() {
        print!("{} ", x[i]);
    }
} */

fn main() {
    let length = 50;
    let mut y = vec![4.; length];
    y[6] = 3.1415;
    y.push(4.89);
    println!("{}, {}, {}", y[6], y[49], y[50]);
    println!("{:?}", y);
}
/* notice this "println!("{:?}, y);":
when debugging the program, it is useful to use this to display the contents of structure, whout having to resort to "for" loop.
*/
