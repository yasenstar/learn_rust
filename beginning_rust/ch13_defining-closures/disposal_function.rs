/* fn main() {
    let mut arr = [ 4, 8, 1, 10, 0, 45, 12, 7];
    println!("{:?}", arr);
    arr.reverse();
    println!("{:?}", arr);
    arr.sort();
    println!("{:?}", arr);
    arr.reverse();
    println!("{:?}", arr);
} */

// Sort in descending order using function
fn main() {
    let mut arr = [ 4, 8, 1, 10, 0, 45, 12, 7];
    use std::cmp::Ordering;
    fn desc(a: &i32, b: &i32) -> Ordering {
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else { Ordering::Equal }
    }
    println!("{:?}", arr);
    arr.sort_by(desc);
    println!("{:?}", arr);
}