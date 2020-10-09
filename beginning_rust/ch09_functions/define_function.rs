// my first guessing try, but have error pop up
/* fn add_number(x,y) {
    return x + y;
}
fn main() {
    let a = 5;
    let b = 4;
    println!("{}", add_number(a, b));
} */

// learning from basic
/* fn f1() { print!("1"); }
fn main() {
    f1();
    fn f2() { print!("2"); }
    f2();
    f1(); f2();
} */

// rewrite the function in the top
/* fn main() {
    let a = 5.;
    let b = 4.;
    add_number(a,b);

    fn add_number(x: f64, y: f64) {
        println!("{} + {} = {}", x, y, x+y);
    }
} */
// it's working, with result "5 + 4 = 9", need to understand how to use "return"

// rewrite with returning a value
fn main() {
    fn add_number(x: f64, y: f64) -> f64 {
        x + y;
    }
    let a = 5.;
    let b = 4.;
    println!("{} + {} = {}", a, b, add_number(a, b));
}