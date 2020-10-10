/* fn main() {
    fn swap_i16_u16(a: i16, b: u16) -> (u16, i16) { (b, a) }
    fn swap_f32_bool(a: f32, b: bool) -> (bool, f32) { (b, a) }
    let x = swap_i16_u16(3i16, 4u16);
    let y = swap_f32_bool(5f32, true);
    print!("{:?} {:?}", x, y);
} */

fn main() {
    fn swap<T1, T2>(a: T1, b: T2) -> (T2, T1) { (b, a) }
    let x = swap('A', 4.5);
    let y = swap(false, 'B');
    let z = swap(3.1415926, 5);
    print!("{:?} {:?} {:?}", x, y, z);
}