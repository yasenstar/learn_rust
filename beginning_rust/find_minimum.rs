fn main() {
    fn times_result(x: f64) -> f64 {
        x * (x+1.) * (x+2.) * (x+3.)
    }
    let x = -2.6 as f64;
    println!("x = {}, result = {}", x, times_result(x));
}