fn main() {
/*     let mut i = 1;
    while i < 100 {
        print!("{} ", i * i);
        i += 1;
    } */

/*     let mut i = 0;
    while i < 50 {
        i += 1;
        if i % 3 != 0 {
            if i * i <= 400 {
                print!("{} ", i * i);
            }
        }
    } */

    let mut i = 0;
    while i < 100 {
        i += 1;
        if i % 3 == 0 {continue;}
        if i * i > 5000 {break;}
        print!("{} ", i * i);
    }

}