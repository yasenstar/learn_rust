# CH09 Defining Functions

## Learning Points

- How to define your own procedures (better known as "functions") and how to invoke them
- When and how you can have several functions with the same name
- How to pass arguments to a function, by-value or by-reference
- How to return simple and composite values from a function
- How to exit prematurely from a function
- How references to objects can be manipulated

## Grammer for Defining a Function

```
fn line() {
    println!("------------");
}
line();
line();
line();
```

Note: you may invoke a function even before defining it, as long as it is defined in the current scope or in an enclosing scope.

Below is valid:

```
f();
fn f() {}
```

While, this code is illegal:

```
a;
let a = 3;
```

While, a function can shadow another function in an outer block, see below sample:

```
fn f() { print!("1"); }
fn main() {
    f(); // Prints 2
    {
        f(); // Prints 3
        fn f() { print!("3"); }
    }
    f(); // Prints 2
    fn f() { print!("2"); }
}
```

This code generates result printing `232`.

## Passing Arguments by Values vs. Reference

| by value | by reference | Memo |
| --- | --- | --- |
| fn double_negatives(mut a: [i32; 10]) -> [32; 10] { | fn double_negatives (a: &mut [i32;10]) { | note: reference have "&" before mut, and not need return type |
| for i in 0..10 { | for i in 0..10 { | same |
| if a[i] < 0 { a[i] *= 2;} } } | if (*a)[i] < 0 { (*a)[i] *= 2; } } } | using "*" before variable to indicate that's one reference |
| let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, -1]; | let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, -1]; | same |
| arr = double_negatives(arr); | double_negatives(&mut arr); | need "&" to indicate reference, and no need put function result to variable again |
| print!("{:?}", arr); } | print!("{:?}", arr); } | same |

