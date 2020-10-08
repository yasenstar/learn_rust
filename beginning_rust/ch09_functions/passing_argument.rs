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

fn main() {
    fn print_double(mut x: f64) {
        x *= 2.;
        print!("{}", x);
    }
    let x = 4.;
    print_double(x);
    print!(" {}", x);
}