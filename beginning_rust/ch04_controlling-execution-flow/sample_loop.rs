// use while
/* fn main() {
    let mut i = 1;
    while true {
        let ii = i * i;
        if ii >= 400 { break; }
        print!("{} ", ii);
        i += 1;
    }
} */

/* result of above code:
    warning: denote infinite loops with `loop { ... }`
    --> sample_loop.rs:4:5
    |
    4 |     while true {
    |     ^^^^^^^^^^ help: use `loop`
    |
    = note: `#[warn(while_true)]` on by default

    warning: 1 warning emitted
Memo: though, the compiler will suggest to denote infinite loops with `loop { ... }`, it's still working to generate results.
*/

// use `loop { ... }` for infinite loops
/* fn main() {
    let mut i = 1;
    loop {
        let ii = i * i;
        if ii >= 600 { break; }
        print!("{} ", ii);
        i += 1;
    }
} */

// counting loops using `for` clause for definite number of loops
/* fn main() {
    for i in 1..11 {
        print!("{} ", i*i*i);
    }
} */

// shalled variable in For
/* fn main() {
    let index = 8;
    for index in 0..4 { print!("{} ", index); }
    print!("Index's value is: {}", index);
} */
/* explanation:
The loop variable is declared by the loop statement, and it is destroyed when the loop ends.
If there were already a variable having the same name, such variable would be shadowed, that is ignored,
for the whole loop, and it would become valid again after the loop, as shown in above code.
*/

// limits with expressions
/* fn main() {
    let mut limit = 10;
    for i in 1..limit {
        limit -= 1;
        println!("i={}, limit={}", i, limit);
    }
    println!(": {}", limit);
} */
/* explanation:
First, the limit variables is created, and it is initialized to the 10 value.
Then, the extremes of the loop are evaluated. The extreme, included, is 1, while the final extreme, excluded is 10.
Therefore the block is executed 9 times; the first one with i=1, the second time whit i=2, and so on.
Each time that the blocks is executed, the value of limit is decremented by 1, there passing from 10 to 1.
However, this does not affect the number of iterations that will be executed.
Note: this is different to C language, refer to code `sample_loop.c`.
*/