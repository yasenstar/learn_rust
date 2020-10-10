// Generic Function sample, similar concept in C++ is "function template", while C doesn't allow generic function
/* fn main() {
    // Library code
    fn f<T>(ch: char, num1: T, num2: T) -> T {
        if ch == 'b' { num1 }
        else { num2 }
    }
    // Application code
    let a: i16 = f::<i16>('a', 37, 41);
    let b: f64 = f::<f64>('b', 37.2, 41.5);
    println!("{} {}", a, b);
} */

// further simplified by inferring the parametric types
/* fn main() {
    // Library code
    fn f<T>(ch: char, num1: T, num2: T) -> T {
        if ch == 'a' { num1 }
        else { num2 }
    }
    // Application code
    let a: i16 = f('a', 37, 41);
    let b: f64 = f('b', 37.2, 41.5);
    print!("{} {}", a, b);
} */

// Parameterize a function with several values of different types
fn main() {
    fn f<Param1, Param2>(_a: Param1, _b: Param2) { println!("hello world") }
    f('a', true);
    f(12.56, "Hello");
    f((3,'a'), [5, 6, 7]);
}