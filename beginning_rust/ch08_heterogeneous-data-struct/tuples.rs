/* fn main() {
    let data = (10000000, 183.19, 'Q', "Yasen");
    let copy_of_data = data;
    print!("{}, {}, {}, {}", data.0, copy_of_data.1, data.3, copy_of_data.2);
} */

// different between Tuple and Array:
// 1. Tuple can include different types, Arry is conposed of objects of the same type
// 2. When declaring, Tuple uses "round parentheses", Array uses "square brackets"
// 3. When accessing the fields, Tuple uses "data.6", while Array uses "arr[6]", different format
// 4. Tuples cannot be accessed by a variable index.

/* fn main() {
    let mut data = (10000000, 183.19, 'Q');
    println!("{:?}", data);
    data.0 = -5;
    data.2 = 'X';
    println!("{} {} {}", data.0, data.1, data.2);
    println!("{:?}", data);
} */
/* note: using {:?} to print all of the data in the structure
Output of above code is:
(10000000, 183.19, 'Q')
-5 183.19 X
(-5, 183.19, 'X')
*/

