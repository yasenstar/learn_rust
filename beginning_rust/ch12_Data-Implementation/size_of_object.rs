/* fn main() {
    print!("{} {}", std::mem::size_of::<i32>(), std::mem::size_of_val(&12));
} */

// The use Directive
/* fn main() {
    use std::mem;
    print!("{} {}", mem::size_of::<i32>(), mem::size_of_val(&12));
} */
// or
/* fn main() {
    use std::mem::size_of;
    use std::mem::size_of_val;;
    print!("{} {}", size_of::<i32>(), size_of_val(&12));
} */
// or
/* fn main() {
    use std::mem::*;
    print!("{} {}", size_of::<i32>(), size_of_val(&12));
} */

