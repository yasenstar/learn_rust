/* fn main() {
    struct S<T1, T2> {
        c: char,
        n1: T1,
        n2: T1,
        n3: T2,
    }
    let _s = S { c: 'a', n1: 34, n2: 782, n3: 0.02 };
    struct SE<T1, T2> (char, T1, T1, T2);
    let _se = SE ('a', 34, 782, 0.02);
} */

/* fn main() {
    struct S<T1, T2> {
        c: char,
        n1: T1,
        n2: T1,
        n3: T2,
    }
    let _s = S::<u16, f32> { c: 'a', n1: 34, n2: 782, n3: 0.02 };

    struct SE<T1, T2> (char, T1, T1, T2);
    let _se = SE::<u16, f32> ( 'a', 34, 782, 0.02 );
} */

// Genericity Mechanics
/* fn main() {
    fn swap<T1, T2>(a: T1, b: T2) -> (T2, T1) { (b, a) }
    let x = swap(3i16, 4u16);
    let y = swap(5f32, true);
    print!("{:?}, {:?}", x, y);
} */

