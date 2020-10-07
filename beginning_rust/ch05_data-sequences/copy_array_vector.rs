// copy an entire array
/* fn main() {
    let mut a1 = [4, 56, -2];
    let a2 = [7, 81, 12500];
    println!("{:?}", a1);
    a1 = a2;
    println!("{:?}", a1);
} */

// copy an entire vector
fn main() {
    let mut a1 = vec![4, 56, -2];
    let a2 = vec![7, 81, 12500];
    println!("{:?}", a1);
    a1 = a2;
    println!("{:?}", a1);
}