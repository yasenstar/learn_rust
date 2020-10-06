fn main() {
    println!("The sum is {}.", 80 + 34);
    println!("{} + {} = {}", 80, 56, 80+56);
    let num_int = (23-6)%5+20*30/(3+4);
    let num_float = (23.-6.)%5.+20.*30./(3.+4.);
    println!("Int: {}, Float: {}", num_int, num_float);
}