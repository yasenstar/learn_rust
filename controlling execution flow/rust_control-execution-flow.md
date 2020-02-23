# Controlling Execution Flow

Key leaning points:

- Use IF statements to execute different statements based on a Boolean condition
- Use IF expressions to generate different values based on a Boolean condition
- Use WHILE statements to repeat some statements as long as a Boolean condition holds
- Use FOR statements to repeat some statements for a definite number of times

## Condition Statements (IF)

Sample: suppose want to print a word, but only if a given number is positive:

```rust
let n = 4;
if n > 0 { print!("positive");}
```

Adding IF and ELSE:

```rust
let n = 0;
if n > 0 {
    print!("nubmer is");
    print!(" positive");
}
else {
    print!("non positive");
}
```

For more than two cases, you can insert IF statements inside blocks, in this way:

```rust
let n = 4;
print!("{}",
    if n > 1000 {
        "big"
    }
    else if n > 0 {
        "small"
    }
    else if n < 0 {
        "negative"
    }
    else {
        "neither positive nor negative"
    }
);
```

The corresponding code in C language is:

```c
#include <stdio.h>
int main(int argc, char **argv) {
    int n = 4;
    printf("%s",
        n > 1000 ?
            "big" :
        n > 0 ?
            "small" :
        n < 0 ?
            "negative" :
            "neither position nor negative");
}
```

